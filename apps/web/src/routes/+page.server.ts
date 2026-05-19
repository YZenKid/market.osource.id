import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api/base';

export const load: PageServerLoad = async ({ fetch, url }) => {
  try {
    const target = apiUrl('/api/install/state');
    const requestUrl = /^https?:\/\//.test(target) ? target : new URL(target, url.origin).toString();
    const res = await fetch(requestUrl);
    if (!res.ok) {
      return { installState: null, apiError: true };
    }
    const data = await res.json().catch(() => null);
    if (!data) return { installState: null, apiError: true };

    if (data.locked) {
      throw redirect(302, '/store');
    }
    throw redirect(302, '/install');
  } catch (err) {
    if (err && typeof err === 'object' && 'status' in err) throw err;
    return { installState: null, apiError: true };
  }
};
