// API client for Rhyph backend
const BASE = '';

let authToken: string | null = null;

export function setToken(token: string | null) {
    authToken = token;
}

async function request(path: string, options: RequestInit = {}): Promise<Response> {
    const headers: Record<string, string> = {
        'Content-Type': 'application/json',
        ...(options.headers as Record<string, string> || {}),
    };
    if (authToken) {
        headers['Authorization'] = `Bearer ${authToken}`;
    }
    const res = await fetch(`${BASE}${path}`, { ...options, headers });
    if (!res.ok) {
        const body = await res.json().catch(() => ({ error: res.statusText }));
        throw new Error(body.error || `HTTP ${res.status}`);
    }
    return res;
}

// Auth
export async function login(email: string, password: string) {
    const res = await request('/api/v1/auth/login', {
        method: 'POST',
        body: JSON.stringify({ email, password }),
    });
    return res.json();
}

// Events
export async function listEvents(orgSlug: string) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events`);
    return res.json();
}

export async function createEvent(orgSlug: string, data: Record<string, unknown>) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events`, {
        method: 'POST',
        body: JSON.stringify(data),
    });
    return res.json();
}

export async function getEvent(orgSlug: string, eventSlug: string) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}`);
    return res.json();
}

// Cart
export async function getCart(orgSlug: string, eventSlug: string) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/cart`);
    return res.json();
}

export async function addToCart(orgSlug: string, eventSlug: string, data: { item_id: string; quantity?: number; variation_id?: string; seat_id?: string }) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/cart`, {
        method: 'POST',
        body: JSON.stringify(data),
    });
    return res.json();
}

export async function removeFromCart(orgSlug: string, eventSlug: string, positionId: string) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/cart/${positionId}`, {
        method: 'DELETE',
    });
    return res.json();
}

// Orders
export async function createOrder(orgSlug: string, eventSlug: string, data: { email?: string; locale?: string }) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/orders`, {
        method: 'POST',
        body: JSON.stringify(data),
    });
    return res.json();
}

export async function getOrderByCode(orgSlug: string, eventSlug: string, code: string) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/orders/${code}`);
    return res.json();
}

// Checkin
export async function getCheckinStats(listId: string) {
    const res = await request(`/api/v1/checkin/lists/${listId}/stats`);
    return res.json();
}

export async function updateEvent(orgSlug: string, eventSlug: string, data: Record<string, unknown>) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}`, {
        method: 'PATCH',
        body: JSON.stringify(data),
    });
    return res.json();
}

// Items (tickets)
export async function listItems(orgSlug: string, eventSlug: string) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/items`);
    return res.json();
}

export async function createItem(orgSlug: string, eventSlug: string, data: Record<string, unknown>) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/items`, {
        method: 'POST',
        body: JSON.stringify(data),
    });
    return res.json();
}

export async function updateItem(orgSlug: string, eventSlug: string, id: string, data: Record<string, unknown>) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/items/${id}`, {
        method: 'PATCH',
        body: JSON.stringify(data),
    });
    return res.json();
}

// Quotas
export async function listQuotas(orgSlug: string, eventSlug: string) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/quotas`);
    return res.json();
}

export async function createQuota(orgSlug: string, eventSlug: string, data: Record<string, unknown>) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/quotas`, {
        method: 'POST',
        body: JSON.stringify(data),
    });
    return res.json();
}

export async function deleteQuota(orgSlug: string, eventSlug: string, id: string) {
    await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/quotas/${id}`, {
        method: 'DELETE',
    });
}

// Checkin Lists
export async function listCheckinLists(orgSlug: string, eventSlug: string) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/checkin-lists`);
    return res.json();
}

export async function createCheckinList(orgSlug: string, eventSlug: string, data: Record<string, unknown>) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/checkin-lists`, {
        method: 'POST',
        body: JSON.stringify(data),
    });
    return res.json();
}

// Devices
export async function createDevice(name: string) {
    const res = await request('/api/v1/admin/devices', {
        method: 'POST',
        body: JSON.stringify({ name }),
    });
    return res.json();
}

export async function listDevices() {
    const res = await request('/api/v1/admin/devices');
    return res.json();
}
