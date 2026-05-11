CREATE TABLE IF NOT EXISTS permissions (
  code text PRIMARY KEY,
  description text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS brand_member_permissions (
  brand_member_id uuid NOT NULL REFERENCES brand_members(id) ON DELETE CASCADE,
  permission_code text NOT NULL REFERENCES permissions(code) ON DELETE RESTRICT,
  granted_by_user_id uuid REFERENCES users(id),
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (brand_member_id, permission_code)
);

INSERT INTO permissions (code, description)
VALUES
  ('payment_proof.view_assigned', 'View private payment proof files only for orders containing an explicitly assigned seller brand.'),
  ('payment_proof.verify', 'Verify uploaded payment proof for an assigned order scope.'),
  ('payment_proof.reject', 'Reject uploaded payment proof for an assigned order scope.')
ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

CREATE INDEX IF NOT EXISTS idx_brand_member_permissions_permission_code
  ON brand_member_permissions(permission_code);
