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

// Setup wizard
export async function needsSetup() {
    const res = await request('/api/v1/auth/needs-setup');
    return res.json() as Promise<{ needs_setup: boolean }>;
}

export async function setup(email: string, password: string) {
    const res = await request('/api/v1/auth/setup', {
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

// Organizer (public config)
export interface OrganizerConfig {
    slug: string;
    name: string;
    theme: Record<string, unknown>;
    custom_domain: string | null;
}

export async function getOrganizer(slug: string): Promise<OrganizerConfig> {
    const res = await request(`/api/v1/organizers/${slug}`);
    return res.json();
}

export interface OrganizerFull {
    id: string;
    slug: string;
    name: string;
    theme: Record<string, unknown>;
    custom_domain: string | null;
    created_at: string;
    updated_at: string;
}

export async function updateOrganizer(slug: string, data: { name?: string; theme?: Record<string, unknown>; custom_domain?: string }): Promise<OrganizerFull> {
    const res = await request(`/api/v1/admin/organizers/${slug}`, {
        method: 'PATCH',
        body: JSON.stringify(data),
    });
    return res.json();
}

// Images
export interface ImageInfo {
    id: string;
    url: string;
    original_name: string;
    content_type: string;
    size_bytes: number;
    created_at: string;
}

export async function uploadImage(slug: string, file: File): Promise<ImageInfo> {
    const form = new FormData();
    form.append('file', file);
    const headers: Record<string, string> = {};
    // Don't set Content-Type — browser sets it with boundary for multipart
    const token = localStorage.getItem('rhyph_token');
    if (token) headers['Authorization'] = `Bearer ${token}`;
    const res = await fetch(`/api/v1/admin/organizers/${slug}/images`, {
        method: 'POST',
        headers,
        body: form,
    });
    if (!res.ok) {
        const body = await res.json().catch(() => ({ error: res.statusText }));
        throw new Error(body.error || `HTTP ${res.status}`);
    }
    return res.json();
}

export async function listImages(slug: string): Promise<ImageInfo[]> {
    const res = await request(`/api/v1/admin/organizers/${slug}/images`);
    return res.json();
}

export async function deleteImage(slug: string, imageId: string): Promise<void> {
    await request(`/api/v1/admin/organizers/${slug}/images/${imageId}`, {
        method: 'DELETE',
    });
}

// Custom SPA
export interface SpaStatus {
    has_custom_spa: boolean;
    index_exists: boolean;
    spa_updated_at: string | null;
}

export async function getSpaStatus(slug: string): Promise<SpaStatus> {
    const res = await request(`/api/v1/admin/organizers/${slug}/spa`);
    return res.json();
}

export async function uploadSpa(slug: string, file: File): Promise<{ ok: boolean; file_count: number; path: string }> {
    const form = new FormData();
    form.append('file', file);
    const headers: Record<string, string> = {};
    const token = localStorage.getItem('rhyph_token');
    if (token) headers['Authorization'] = `Bearer ${token}`;
    const res = await fetch(`/api/v1/admin/organizers/${slug}/spa`, {
        method: 'POST',
        headers,
        body: form,
    });
    if (!res.ok) {
        const body = await res.json().catch(() => ({ error: res.statusText }));
        throw new Error(body.error || `HTTP ${res.status}`);
    }
    return res.json();
}

export async function deleteSpa(slug: string): Promise<void> {
    await request(`/api/v1/admin/organizers/${slug}/spa`, { method: 'DELETE' });
}
