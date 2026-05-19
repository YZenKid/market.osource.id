import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { apiUrl } from '$lib/api/base';

export const load: LayoutServerLoad = async ({ fetch, url }) => {
  const target = apiUrl('/api/install/state');
  const requestUrl = /^https?:\/\//.test(target) ? target : new URL(target, url.origin).toString();
  const res = await fetch(requestUrl).catch(() => null);

  if (!res?.ok) {
    return { installState: null };
  }

  const state = await res.json().catch(() => null);

  if (state && !state.locked && url.pathname === '/admin/login') {
    return { installState: state };
  }

  if (state && !state.locked) {
    throw redirect(302, '/install');
  }

  return { installState: state };
};
