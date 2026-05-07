CREATE TABLE IF NOT EXISTS packages (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  package_id text NOT NULL UNIQUE,
  name text NOT NULL,
  version text NOT NULL,
  core_version_range text NOT NULL,
  license_status text NOT NULL DEFAULT 'unlicensed',
  enabled boolean NOT NULL DEFAULT false,
  installed_at timestamptz,
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS package_migrations (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  package_row_id uuid REFERENCES packages(id),
  package_id text NOT NULL,
  package_version text NOT NULL,
  migration_version text NOT NULL,
  checksum text NOT NULL,
  applied_at timestamptz NOT NULL DEFAULT now(),
  UNIQUE (package_id, migration_version)
);

CREATE TABLE IF NOT EXISTS audit_events (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  actor_user_id uuid REFERENCES users(id),
  action text NOT NULL,
  target_type text NOT NULL,
  target_id uuid,
  result text NOT NULL,
  metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
  ip_address inet,
  user_agent text,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_packages_enabled ON packages(enabled);
CREATE INDEX IF NOT EXISTS idx_audit_actor_user_id ON audit_events(actor_user_id);
CREATE INDEX IF NOT EXISTS idx_audit_target ON audit_events(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_audit_created_at ON audit_events(created_at);
