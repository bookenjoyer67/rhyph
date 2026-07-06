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

// Orders
export async function getOrderByCode(orgSlug: string, eventSlug: string, code: string) {
    const res = await request(`/api/v1/organizers/${orgSlug}/events/${eventSlug}/orders/${code}`);
    return res.json();
}

// Checkin
export async function getCheckinStats(listId: string) {
    const res = await request(`/api/v1/checkin/lists/${listId}/stats`);
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
