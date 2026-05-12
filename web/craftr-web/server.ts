import express from "express";
import { createServer as createViteServer } from "vite";
import path from "path";
import { fileURLToPath } from "url";
import Stripe from "stripe";
import { createClient } from "@supabase/supabase-js";
import { Resend } from "resend";
import dotenv from "dotenv";

dotenv.config();

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const app = express();
const PORT = 3000;

// Initialize clients lazily or with checks
const getStripe = () => {
  if (!process.env.STRIPE_SECRET_KEY) return null;
  return new Stripe(process.env.STRIPE_SECRET_KEY);
};

const getSupabase = () => {
  if (!process.env.VITE_SUPABASE_URL || !process.env.SUPABASE_SERVICE_ROLE_KEY) return null;
  return createClient(process.env.VITE_SUPABASE_URL, process.env.SUPABASE_SERVICE_ROLE_KEY);
};

const getResend = () => {
  if (!process.env.RESEND_API_KEY) return null;
  return new Resend(process.env.RESEND_API_KEY);
};

// Middleware for Stripe Webhook (needs raw body)
app.post("/api/stripe/webhook", express.raw({ type: "application/json" }), async (req, res) => {
  const stripe = getStripe();
  const sig = req.headers["stripe-signature"];
  const webhookSecret = process.env.STRIPE_WEBHOOK_SECRET;

  if (!stripe || !sig || !webhookSecret) {
    return res.status(400).send("Webhook Error: Missing Stripe configuration");
  }

  let event;

  try {
    event = stripe.webhooks.constructEvent(req.body, sig, webhookSecret);
  } catch (err: any) {
    console.error(`Webhook signature verification failed.`, err.message);
    return res.status(400).send(`Webhook Error: ${err.message}`);
  }

  // Handle the event
  if (event.type === "checkout.session.completed") {
    const session = event.data.object as Stripe.Checkout.Session;
    const email = session.customer_details?.email;
    const plan = session.metadata?.plan || 'pro';
    
    if (email) {
      const supabase = getSupabase();
      if (supabase) {
        // Generate license key: CRFTR-XXXX-XXXX-XXXX
        const licenseKey = `CRFTR-${Math.random().toString(36).substring(2, 6).toUpperCase()}-${Math.random().toString(36).substring(2, 6).toUpperCase()}-${Math.random().toString(36).substring(2, 6).toUpperCase()}`;
        
        const { error } = await supabase.from("licenses").upsert({
          email,
          license_key: licenseKey,
          plan,
          stripe_customer_id: session.customer as string,
          created_at: new Date().toISOString()
        }, { onConflict: 'email' });

        if (!error) {
          const resend = getResend();
          if (resend) {
            await resend.emails.send({
              from: 'Craftr <noreply@craftr.app>',
              to: [email],
              subject: 'Your Craftr Pro License Key',
              html: `<h1>Welcome to Craftr Pro!</h1><p>Your license key is: <strong>${licenseKey}</strong></p><p>Activate it in the app settings.</p>`
            });
          }
        }
      }
    }
  }

  res.json({ received: true });
});

// JSON body parser for other routes
app.use(express.json());

// API Routes
app.post("/api/validate", async (req, res) => {
  const { license_key } = req.body;
  const supabase = getSupabase();
  
  if (!supabase) return res.status(500).json({ error: "DB connection failed" });

  const { data, error } = await supabase
    .from("licenses")
    .select("*")
    .eq("license_key", license_key)
    .single();

  if (error || !data) {
    return res.status(404).json({ valid: false });
  }

  // Simple validation logic
  res.json({ 
    valid: true, 
    plan: data.plan, 
    email: data.email,
    expiry: "2099-01-01" // Simplified for lifetime/pro
  });
});

app.post("/api/stripe/checkout", async (req, res) => {
  const { plan, email } = req.body;
  const stripe = getStripe();
  
  if (!stripe) return res.status(500).json({ error: "Stripe connection failed" });

  const priceId = plan === 'lifetime' 
    ? process.env.STRIPE_LIFETIME_PRICE_ID 
    : process.env.STRIPE_PRO_PRICE_ID;

  if (!priceId) return res.status(400).json({ error: "Invalid plan" });

  try {
    const session = await stripe.checkout.sessions.create({
      payment_method_types: ["card"],
      line_items: [{ price: priceId, quantity: 1 }],
      mode: plan === 'lifetime' ? "payment" : "subscription",
      customer_email: email,
      success_url: `${process.env.APP_URL}/success?session_id={CHECKOUT_SESSION_ID}`,
      cancel_url: `${process.env.APP_URL}/pricing`,
      metadata: { plan }
    });

    res.json({ id: session.id, url: session.url });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

async function startServer() {
  // Vite middleware for development
  if (process.env.NODE_ENV !== "production") {
    const vite = await createViteServer({
      server: { middlewareMode: true },
      appType: "spa",
    });
    app.use(vite.middlewares);
  } else {
    const distPath = path.join(process.cwd(), "dist");
    app.use(express.static(distPath));
    app.get("*", (req, res) => {
      res.sendFile(path.join(distPath, "index.html"));
    });
  }

  app.listen(PORT, "0.0.0.0", () => {
    console.log(`Server running on http://localhost:${PORT}`);
  });
}

startServer();
