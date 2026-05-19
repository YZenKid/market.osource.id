import { env } from '$env/dynamic/public';

export function apiUrl(path: string): string {
  if (/^https?:\/\//.test(path)) return path;
  const normalizedPath = path.startsWith('/') ? path : `/${path}`;
  const base = env.PUBLIC_API_BASE_URL?.trim();
  if (!base) return normalizedPath;
  return `${base.replace(/\/$/, '')}${normalizedPath}`;
}

/** Cached CSRF token per page load. Cleared on navigation by calling clearCsrfCache(). */
let _csrfCache: string | null = null;

export function clearCsrfCache() {
  _csrfCache = null;
}

/**
 * Fetch and cache the CSRF token for the current session.
 * Throws if the endpoint is unavailable or returns an invalid token.
 */
export async function getCsrfToken(): Promise<string> {
  if (_csrfCache) return _csrfCache;
  const res = await fetch(apiUrl('/api/auth/csrf'), { credentials: 'include' });
  const payload = await res.json().catch(() => ({}));
  if (!res.ok || typeof payload.token !== 'string') {
    throw new Error(payload.message ?? 'Gagal menyiapkan token keamanan.');
  }
  _csrfCache = payload.token as string;
  return _csrfCache;
}

export type ApiFetchOptions = RequestInit & {
  /** If true, automatically fetches and attaches X-CSRF-Token header. */
  csrf?: boolean;
};

/**
 * Thin wrapper around fetch that:
 * - Resolves the URL via apiUrl()
 * - Always sends credentials: 'include'
 * - Optionally attaches CSRF token
 * - Returns { ok, status, data } — never throws on HTTP errors
 */
export async function apiFetch<T = unknown>(
  path: string,
  options: ApiFetchOptions = {}
): Promise<{ ok: boolean; status: number; data: T | null; message: string }> {
  const { csrf, headers: extraHeaders, ...rest } = options;

  const headers: Record<string, string> = {
    ...(extraHeaders as Record<string, string>)
  };

  if (csrf) {
    try {
      headers['x-csrf-token'] = await getCsrfToken();
    } catch (err) {
      return {
        ok: false,
        status: 0,
        data: null,
        message: err instanceof Error ? err.message : 'CSRF token unavailable.'
      };
    }
  }

  if (rest.body && typeof rest.body === 'string' && !headers['content-type']) {
    headers['content-type'] = 'application/json';
  }

  try {
    const res = await fetch(apiUrl(path), {
      credentials: 'include',
      headers,
      ...rest
    });

    let data: T | null = null;
    let message = '';
    try {
      const json = await res.json();
      data = json as T;
      message = (json as any)?.message ?? '';
    } catch {
      // non-JSON response
    }

    return { ok: res.ok, status: res.status, data, message };
  } catch (err) {
    return {
      ok: false,
      status: 0,
      data: null,
      message: err instanceof Error ? err.message : 'Network error.'
    };
  }
}
