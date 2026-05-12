-- 1. Users + License table
CREATE TABLE licenses (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  email TEXT UNIQUE NOT NULL,
  license_key TEXT UNIQUE NOT NULL,
  -- format: CRFTR-XXXX-XXXX-XXXX
  plan TEXT NOT NULL DEFAULT 'free',
  -- values: 'free' | 'pro' | 'lifetime'
  status TEXT NOT NULL DEFAULT 'active',
  -- values: 'active' | 'cancelled' | 'expired'
  stripe_customer_id TEXT,
  stripe_subscription_id TEXT,
  requests_today INTEGER DEFAULT 0,
  last_request_date DATE,
  activated_at TIMESTAMP WITH TIME ZONE,
  expires_at TIMESTAMP WITH TIME ZONE,
  -- null for lifetime
  created_at TIMESTAMP WITH TIME ZONE 
    DEFAULT NOW()
);

-- 2. Payments history table  
CREATE TABLE payments (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  email TEXT NOT NULL,
  license_key TEXT NOT NULL,
  plan TEXT NOT NULL,
  amount DECIMAL(10,2) NOT NULL,
  currency TEXT DEFAULT 'usd',
  stripe_payment_id TEXT UNIQUE,
  stripe_customer_id TEXT,
  status TEXT DEFAULT 'completed',
  created_at TIMESTAMP WITH TIME ZONE 
    DEFAULT NOW()
);

-- 3. App usage analytics table
CREATE TABLE usage_logs (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  license_key TEXT,
  action TEXT NOT NULL,
  -- values: 'enhance' | 'compress'
  plan TEXT NOT NULL,
  -- values: 'free' | 'pro' | 'lifetime'
  os TEXT,
  -- values: 'windows' | 'macos'
  app_version TEXT,
  created_at TIMESTAMP WITH TIME ZONE 
    DEFAULT NOW()
);

-- Indexes for fast lookups
CREATE INDEX idx_licenses_key 
  ON licenses(license_key);
CREATE INDEX idx_licenses_email 
  ON licenses(email);
CREATE INDEX idx_payments_email 
  ON payments(email);
