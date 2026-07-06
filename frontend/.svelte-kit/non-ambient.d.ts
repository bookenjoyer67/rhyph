
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	type MatcherParam<M> = M extends (param : string) => param is (infer U extends string) ? U : string;

	export interface AppTypes {
		RouteId(): "/" | "/admin" | "/admin/checkin" | "/admin/devices" | "/admin/events" | "/admin/events/[slug]" | "/admin/orders" | "/cart" | "/events" | "/events/[org]" | "/events/[org]/[slug]" | "/login" | "/orders" | "/orders/[code]" | "/scan";
		RouteParams(): {
			"/admin/events/[slug]": { slug: string };
			"/events/[org]": { org: string };
			"/events/[org]/[slug]": { org: string; slug: string };
			"/orders/[code]": { code: string }
		};
		LayoutParams(): {
			"/": { slug?: string | undefined; org?: string | undefined; code?: string | undefined };
			"/admin": { slug?: string | undefined };
			"/admin/checkin": Record<string, never>;
			"/admin/devices": Record<string, never>;
			"/admin/events": { slug?: string | undefined };
			"/admin/events/[slug]": { slug: string };
			"/admin/orders": Record<string, never>;
			"/cart": Record<string, never>;
			"/events": { org?: string | undefined; slug?: string | undefined };
			"/events/[org]": { org: string; slug?: string | undefined };
			"/events/[org]/[slug]": { org: string; slug: string };
			"/login": Record<string, never>;
			"/orders": { code?: string | undefined };
			"/orders/[code]": { code: string };
			"/scan": Record<string, never>
		};
		Pathname(): "/" | "/admin/checkin" | "/admin/devices" | "/admin/events" | `/admin/events/${string}` & {} | "/admin/orders" | "/cart" | `/events/${string}/${string}` & {} | "/login" | `/orders/${string}` & {} | "/scan";
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): string & {};
	}
}