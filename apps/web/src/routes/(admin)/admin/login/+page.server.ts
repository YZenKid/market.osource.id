import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api/base';

export const load: PageServerLoad = async ({ fetch, request, url }) => {
  const cookie = request.headers.get('cookie') ?? '';

  const authTarget = apiUrl('/api/auth/me');
  const authUrl = /^https?:\/\//.test(authTarget) ? authTarget : new URL(authTarget, url.origin).toString();
  const authRes = await fetch(authUrl, {
    headers: cookie ? { cookie } : undefined
  }).catch(() => null);

  if (authRes?.ok) {
    throw redirect(302, '/admin');
  }

  const installTarget = apiUrl('/api/install/state');
  const installUrl = /^https?:\/\//.test(installTarget) ? installTarget : new URL(installTarget, url.origin).toString();
  const installRes = await fetch(installUrl).catch(() => null);
  const installState = installRes?.ok ? await installRes.json().catch(() => null) : null;

  return {
    installLocked: Boolean(installState?.locked),
    installState
  };
};
