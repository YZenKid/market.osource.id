import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api/base';

export const load: PageServerLoad = async ({ fetch, request, url }) => {
  const cookie = request.headers.get('cookie') ?? '';

  const authTarget = apiUrl('/api/auth/me');
  const authUrl = /^https?:\/\//.test(authTarget)
    ? authTarget
    : new URL(authTarget, url.origin).toString();

  const authRes = await fetch(authUrl, {
    headers: cookie ? { cookie } : undefined
  }).catch(() => null);

  if (!authRes?.ok) {
    throw redirect(302, '/admin/login');
  }

  const me = await authRes.json().catch(() => null);
  const roleCode: string = me?.user?.role?.code ?? '';

  if (roleCode !== 'admin' && roleCode !== 'super_admin') {
    throw redirect(302, '/admin/login');
  }

  return { roleCode };
};
