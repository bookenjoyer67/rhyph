// API client for The Neon Cathedral — talks to the Rhyph backend
const BASE = '';

let authToken: string | null = null;

export function setToken(token: string | null) { authToken = token; }

async function request(path: string, opts: RequestInit = {}): Promise<Response> {
    const headers: Record<string, string> = { 'Content-Type': 'application/json', ...(opts.headers as Record<string, string> || {}) };
    if (authToken) headers['Authorization'] = `Bearer ${authToken}`;
    const res = await fetch(`${BASE}${path}`, { ...opts, headers });
    if (!res.ok) {
        const body = await res.json().catch(() => ({ error: res.statusText }));
        throw new Error(body.error || `HTTP ${res.status}`);
    }
    return res;
}

// Organizer
export async function getOrganizer(slug: string) {
    return (await request(`/api/v1/organizers/${slug}`)).json();
}

// Events
export async function listEvents(orgSlug: string) {
    return (await request(`/api/v1/organizers/${orgSlug}/events`)).json();
}
export async function getEvent(orgSlug: string, eventSlug: string) {
    return (await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}`)).json();
}
export async function listItems(orgSlug: string, eventSlug: string) {
    return (await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/items`)).json();
}

// Cart
export async function getCart(orgSlug: string, eventSlug: string) {
    return (await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/cart`)).json();
}
export async function addToCart(orgSlug: string, eventSlug: string, data: { item_id: string; quantity?: number }) {
    return (await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/cart`, { method: 'POST', body: JSON.stringify(data) })).json();
}
export async function removeFromCart(orgSlug: string, eventSlug: string, positionId: string) {
    return (await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/cart/${positionId}`, { method: 'DELETE' })).json();
}

// Orders
export async function createOrder(orgSlug: string, eventSlug: string, data?: { email?: string }) {
    return (await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/orders`, { method: 'POST', body: JSON.stringify(data || {}) })).json();
}
export async function getOrderByCode(orgSlug: string, eventSlug: string, code: string) {
    return (await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/orders/${code}`)).json();
}
