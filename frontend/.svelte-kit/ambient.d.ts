
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * This module provides access to environment variables that are injected _statically_ into your bundle at build time and are limited to _private_ access.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Static environment variables are [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env` at build time and then statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * **_Private_ access:**
 * 
 * - This module cannot be imported into client-side code
 * - This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured)
 * 
 * For example, given the following build time environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { ENVIRONMENT, PUBLIC_BASE_URL } from '$env/static/private';
 * 
 * console.log(ENVIRONMENT); // => "production"
 * console.log(PUBLIC_BASE_URL); // => throws error during build
 * ```
 * 
 * The above values will be the same _even if_ different values for `ENVIRONMENT` or `PUBLIC_BASE_URL` are set at runtime, as they are statically replaced in your code with their build time values.
 */
declare module '$env/static/private' {
	export const NODE_ENV: string;
	export const HYPRCURSOR_SIZE: string;
	export const OLDPWD: string;
	export const npm_node_execpath: string;
	export const TERMINAL_LIFETIME_SECONDS: string;
	export const ENVMAN_LOAD: string;
	export const MOA_TOOLS_DEBUG: string;
	export const WAYLAND_DISPLAY: string;
	export const BROWSER_INACTIVITY_TIMEOUT: string;
	export const STARSHIP_SHELL: string;
	export const MOTD_SHOWN: string;
	export const TERMINAL_CONTAINER_DISK: string;
	export const LC_ADDRESS: string;
	export const _JAVA_AWT_WM_NONREPARENTING: string;
	export const HOME: string;
	export const GDM_LANG: string;
	export const _: string;
	export const AUXILIARY_VISION_PROVIDER: string;
	export const WEB_TOOLS_DEBUG: string;
	export const TERMINAL_MODAL_IMAGE: string;
	export const npm_package_version: string;
	export const BROWSER_SESSION_TIMEOUT: string;
	export const npm_config_global_prefix: string;
	export const IMAGE_TOOLS_DEBUG: string;
	export const XCURSOR_THEME: string;
	export const KITTY_PUBLIC_KEY: string;
	export const TERMINAL_CONTAINER_CPU: string;
	export const TERMINAL_DOCKER_EXTRA_ARGS: string;
	export const MATRIX_E2EE_MODE: string;
	export const LC_TIME: string;
	export const KITTY_PID: string;
	export const npm_command: string;
	export const HERMES_SESSION_ID: string;
	export const npm_config_userconfig: string;
	export const XDG_SESSION_TYPE: string;
	export const LANG: string;
	export const TERMINAL_TIMEOUT: string;
	export const XDG_BACKEND: string;
	export const QT_WAYLAND_DISABLE_WINDOWDECORATION: string;
	export const XDG_SESSION_DESKTOP: string;
	export const AQ_DRM_DEVICES: string;
	export const TERMINAL_DOCKER_RUN_AS_HOST_USER: string;
	export const npm_config_init_module: string;
	export const HYPRLAND_CMD: string;
	export const GTK_THEME: string;
	export const XDG_SESSION_EXTRA_DEVICE_ACCESS: string;
	export const LC_PAPER: string;
	export const HERMES_KANBAN_BOARD: string;
	export const EDITOR: string;
	export const LC_TELEPHONE: string;
	export const NODE: string;
	export const npm_config_cache: string;
	export const TERM: string;
	export const COLORTERM: string;
	export const MAIL: string;
	export const MOZ_ENABLE_WAYLAND: string;
	export const LC_NAME: string;
	export const npm_lifecycle_event: string;
	export const PWD: string;
	export const LOGNAME: string;
	export const SHELL: string;
	export const npm_config_node_gyp: string;
	export const MATRIX_HOME_ROOM: string;
	export const XCURSOR_PATH: string;
	export const COLOR: string;
	export const npm_config_globalconfig: string;
	export const DESKTOP_SESSION: string;
	export const npm_execpath: string;
	export const TERMINAL_ENV: string;
	export const VSSCRIPT_PATH: string;
	export const HERMES_QUIET: string;
	export const HERMES_INTERACTIVE: string;
	export const TERMINAL_DOCKER_ENV: string;
	export const HL_INITIAL_WORKSPACE_TOKEN: string;
	export const VIRTUAL_ENV_DISABLE_PROMPT: string;
	export const USER: string;
	export const KITTY_WINDOW_ID: string;
	export const MANROFFOPT: string;
	export const LC_NUMERIC: string;
	export const AUXILIARY_WEB_EXTRACT_PROVIDER: string;
	export const DEBUGINFOD_URLS: string;
	export const STARSHIP_SESSION_KEY: string;
	export const INIT_CWD: string;
	export const QT_QPA_PLATFORM: string;
	export const VISION_TOOLS_DEBUG: string;
	export const XCURSOR_SIZE: string;
	export const npm_lifecycle_script: string;
	export const npm_config_npm_version: string;
	export const XDG_SESSION_CLASS: string;
	export const TERMINFO: string;
	export const TERMINAL_DAYTONA_IMAGE: string;
	export const AUXILIARY_APPROVAL_PROVIDER: string;
	export const LC_IDENTIFICATION: string;
	export const npm_config_local_prefix: string;
	export const npm_package_name: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const LC_MEASUREMENT: string;
	export const LC_MONETARY: string;
	export const npm_config_prefix: string;
	export const TERMINAL_DOCKER_FORWARD_ENV: string;
	export const TERMINAL_SINGULARITY_IMAGE: string;
	export const CUDA_PATH: string;
	export const HYPRLAND_INSTANCE_SIGNATURE: string;
	export const CUDA_DISABLE_PERF_BOOST: string;
	export const MANPAGER: string;
	export const XDG_DATA_DIRS: string;
	export const TERMINAL_DOCKER_IMAGE: string;
	export const DISPLAY: string;
	export const SHLVL: string;
	export const XDG_SEAT: string;
	export const MATRIX_HOME_ROOM_THREAD_ID: string;
	export const XDG_VTNR: string;
	export const XDG_SESSION_ID: string;
	export const TERMINAL_PERSISTENT_SHELL: string;
	export const npm_config_user_agent: string;
	export const TERMINAL_HOME_MODE: string;
	export const TERMINAL_CWD: string;
	export const XDG_RUNTIME_DIR: string;
	export const BROWSERBASE_PROXIES: string;
	export const NVCC_CCBIN: string;
	export const npm_package_json: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const TERMINAL_DOCKER_VOLUMES: string;
	export const PATH: string;
	export const HERMES_REDACT_SECRETS: string;
	export const GDK_BACKEND: string;
	export const SVELTEKIT_FORK: string;
	export const TERMINAL_DOCKER_MOUNT_CWD_TO_WORKSPACE: string;
	export const npm_config_allow_scripts: string;
	export const GDMSESSION: string;
	export const npm_config_noproxy: string;
	export const KITTY_INSTALLATION_DIR: string;
	export const HERMES_REAL_HOME: string;
	export const TERMINAL_CONTAINER_MEMORY: string;
	export const BROWSERBASE_ADVANCED_STEALTH: string;
	export const USERNAME: string;
	export const TERMINAL_CONTAINER_PERSISTENT: string;
}

/**
 * This module provides access to environment variables that are injected _statically_ into your bundle at build time and are _publicly_ accessible.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Static environment variables are [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env` at build time and then statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * **_Public_ access:**
 * 
 * - This module _can_ be imported into client-side code
 * - **Only** variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`) are included
 * 
 * For example, given the following build time environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { ENVIRONMENT, PUBLIC_BASE_URL } from '$env/static/public';
 * 
 * console.log(ENVIRONMENT); // => throws error during build
 * console.log(PUBLIC_BASE_URL); // => "http://site.com"
 * ```
 * 
 * The above values will be the same _even if_ different values for `ENVIRONMENT` or `PUBLIC_BASE_URL` are set at runtime, as they are statically replaced in your code with their build time values.
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to environment variables set _dynamically_ at runtime and that are limited to _private_ access.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Dynamic environment variables are defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`.
 * 
 * **_Private_ access:**
 * 
 * - This module cannot be imported into client-side code
 * - This module includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured)
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 * 
 * > [!NOTE] To get correct types, environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * >
 * > ```env
 * > MY_FEATURE_FLAG=
 * > ```
 * >
 * > You can override `.env` values from the command line like so:
 * >
 * > ```sh
 * > MY_FEATURE_FLAG="enabled" npm run dev
 * > ```
 * 
 * For example, given the following runtime environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * 
 * console.log(env.ENVIRONMENT); // => "production"
 * console.log(env.PUBLIC_BASE_URL); // => undefined
 * ```
 */
declare module '$env/dynamic/private' {
	export const env: {
		NODE_ENV: string;
		HYPRCURSOR_SIZE: string;
		OLDPWD: string;
		npm_node_execpath: string;
		TERMINAL_LIFETIME_SECONDS: string;
		ENVMAN_LOAD: string;
		MOA_TOOLS_DEBUG: string;
		WAYLAND_DISPLAY: string;
		BROWSER_INACTIVITY_TIMEOUT: string;
		STARSHIP_SHELL: string;
		MOTD_SHOWN: string;
		TERMINAL_CONTAINER_DISK: string;
		LC_ADDRESS: string;
		_JAVA_AWT_WM_NONREPARENTING: string;
		HOME: string;
		GDM_LANG: string;
		_: string;
		AUXILIARY_VISION_PROVIDER: string;
		WEB_TOOLS_DEBUG: string;
		TERMINAL_MODAL_IMAGE: string;
		npm_package_version: string;
		BROWSER_SESSION_TIMEOUT: string;
		npm_config_global_prefix: string;
		IMAGE_TOOLS_DEBUG: string;
		XCURSOR_THEME: string;
		KITTY_PUBLIC_KEY: string;
		TERMINAL_CONTAINER_CPU: string;
		TERMINAL_DOCKER_EXTRA_ARGS: string;
		MATRIX_E2EE_MODE: string;
		LC_TIME: string;
		KITTY_PID: string;
		npm_command: string;
		HERMES_SESSION_ID: string;
		npm_config_userconfig: string;
		XDG_SESSION_TYPE: string;
		LANG: string;
		TERMINAL_TIMEOUT: string;
		XDG_BACKEND: string;
		QT_WAYLAND_DISABLE_WINDOWDECORATION: string;
		XDG_SESSION_DESKTOP: string;
		AQ_DRM_DEVICES: string;
		TERMINAL_DOCKER_RUN_AS_HOST_USER: string;
		npm_config_init_module: string;
		HYPRLAND_CMD: string;
		GTK_THEME: string;
		XDG_SESSION_EXTRA_DEVICE_ACCESS: string;
		LC_PAPER: string;
		HERMES_KANBAN_BOARD: string;
		EDITOR: string;
		LC_TELEPHONE: string;
		NODE: string;
		npm_config_cache: string;
		TERM: string;
		COLORTERM: string;
		MAIL: string;
		MOZ_ENABLE_WAYLAND: string;
		LC_NAME: string;
		npm_lifecycle_event: string;
		PWD: string;
		LOGNAME: string;
		SHELL: string;
		npm_config_node_gyp: string;
		MATRIX_HOME_ROOM: string;
		XCURSOR_PATH: string;
		COLOR: string;
		npm_config_globalconfig: string;
		DESKTOP_SESSION: string;
		npm_execpath: string;
		TERMINAL_ENV: string;
		VSSCRIPT_PATH: string;
		HERMES_QUIET: string;
		HERMES_INTERACTIVE: string;
		TERMINAL_DOCKER_ENV: string;
		HL_INITIAL_WORKSPACE_TOKEN: string;
		VIRTUAL_ENV_DISABLE_PROMPT: string;
		USER: string;
		KITTY_WINDOW_ID: string;
		MANROFFOPT: string;
		LC_NUMERIC: string;
		AUXILIARY_WEB_EXTRACT_PROVIDER: string;
		DEBUGINFOD_URLS: string;
		STARSHIP_SESSION_KEY: string;
		INIT_CWD: string;
		QT_QPA_PLATFORM: string;
		VISION_TOOLS_DEBUG: string;
		XCURSOR_SIZE: string;
		npm_lifecycle_script: string;
		npm_config_npm_version: string;
		XDG_SESSION_CLASS: string;
		TERMINFO: string;
		TERMINAL_DAYTONA_IMAGE: string;
		AUXILIARY_APPROVAL_PROVIDER: string;
		LC_IDENTIFICATION: string;
		npm_config_local_prefix: string;
		npm_package_name: string;
		XDG_CURRENT_DESKTOP: string;
		LC_MEASUREMENT: string;
		LC_MONETARY: string;
		npm_config_prefix: string;
		TERMINAL_DOCKER_FORWARD_ENV: string;
		TERMINAL_SINGULARITY_IMAGE: string;
		CUDA_PATH: string;
		HYPRLAND_INSTANCE_SIGNATURE: string;
		CUDA_DISABLE_PERF_BOOST: string;
		MANPAGER: string;
		XDG_DATA_DIRS: string;
		TERMINAL_DOCKER_IMAGE: string;
		DISPLAY: string;
		SHLVL: string;
		XDG_SEAT: string;
		MATRIX_HOME_ROOM_THREAD_ID: string;
		XDG_VTNR: string;
		XDG_SESSION_ID: string;
		TERMINAL_PERSISTENT_SHELL: string;
		npm_config_user_agent: string;
		TERMINAL_HOME_MODE: string;
		TERMINAL_CWD: string;
		XDG_RUNTIME_DIR: string;
		BROWSERBASE_PROXIES: string;
		NVCC_CCBIN: string;
		npm_package_json: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		TERMINAL_DOCKER_VOLUMES: string;
		PATH: string;
		HERMES_REDACT_SECRETS: string;
		GDK_BACKEND: string;
		SVELTEKIT_FORK: string;
		TERMINAL_DOCKER_MOUNT_CWD_TO_WORKSPACE: string;
		npm_config_allow_scripts: string;
		GDMSESSION: string;
		npm_config_noproxy: string;
		KITTY_INSTALLATION_DIR: string;
		HERMES_REAL_HOME: string;
		TERMINAL_CONTAINER_MEMORY: string;
		BROWSERBASE_ADVANCED_STEALTH: string;
		USERNAME: string;
		TERMINAL_CONTAINER_PERSISTENT: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * This module provides access to environment variables set _dynamically_ at runtime and that are _publicly_ accessible.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Dynamic environment variables are defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`.
 * 
 * **_Public_ access:**
 * 
 * - This module _can_ be imported into client-side code
 * - **Only** variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`) are included
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 * 
 * > [!NOTE] To get correct types, environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * >
 * > ```env
 * > MY_FEATURE_FLAG=
 * > ```
 * >
 * > You can override `.env` values from the command line like so:
 * >
 * > ```sh
 * > MY_FEATURE_FLAG="enabled" npm run dev
 * > ```
 * 
 * For example, given the following runtime environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://example.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.ENVIRONMENT); // => undefined, not public
 * console.log(env.PUBLIC_BASE_URL); // => "http://example.com"
 * ```
 * 
 * ```
 * 
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
