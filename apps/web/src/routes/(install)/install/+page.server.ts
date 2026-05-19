import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api/base';

export const load: PageServerLoad = async ({ fetch, url }) => {
  const target = apiUrl('/api/install/state');
  const requestUrl = /^https?:\/\//.test(target) ? target : new URL(target, url.origin).toString();
  const res = await fetch(requestUrl).catch(() => null);
  if (!res?.ok) {
    return { locked: false, gatingUnavailable: true };
  }

  const state = await res.json().catch(() => null);
  if (state?.locked) {
    throw redirect(302, '/store');
  }

  return {
    locked: Boolean(state?.locked),
    installState: state,
    gatingUnavailable: false
  };
};
