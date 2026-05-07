CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS installation_state (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  state text NOT NULL,
  runtime_mode text NOT NULL,
  core_version text NOT NULL,
  installed_at timestamptz,
  locked_at timestamptz,
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS marketplace_settings (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  marketplace_name text NOT NULL,
  base_url text NOT NULL,
  support_contact text,
  default_currency text NOT NULL DEFAULT 'IDR',
  manual_payment_instructions jsonb NOT NULL DEFAULT '{}'::jsonb,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS storage_settings (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  active_provider text NOT NULL DEFAULT 'local',
  local_storage_path text NOT NULL,
  s3_config_masked jsonb NOT NULL DEFAULT '{}'::jsonb,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS tunnel_settings (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  enabled boolean NOT NULL DEFAULT false,
  provider text NOT NULL DEFAULT 'cloudflared',
  public_url text,
  credential_ref text,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);
