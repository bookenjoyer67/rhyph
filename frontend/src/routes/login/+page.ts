export const ssr = false;

export async function load({ fetch }) {
    try {
        const res = await fetch('/api/v1/auth/needs-setup');
        const data = await res.json();
        return { needsSetup: data.needs_setup };
    } catch {
        return { needsSetup: false };
    }
}
