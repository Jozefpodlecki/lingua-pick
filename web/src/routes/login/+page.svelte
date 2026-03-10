<script lang="ts">
    import { setAppContext, loginWithCreds, loginWithWindows, getCurrentProfile } from "$lib/api/app";
    import { appName } from "$lib/constants";
    import type { Context, UserProfile } from "$lib/types/app";
    import type { MouseEventHandler } from "svelte/elements";
    import { error } from '@tauri-apps/plugin-log';
    import { goto } from "$app/navigation";

    let state = $state({
        username: "",
        password: ""
    });

    const isValid = $derived(state.username.trim() !== "" && state.password.trim() !== "");

    const onSubmit: MouseEventHandler<HTMLElement> = async (event) => {
        const { action } = event.currentTarget.dataset;

        try {
            let context: Context;
            let profile: UserProfile | null = null;

            if(action === "login") {
                const token = await loginWithCreds(state);
                profile = await getCurrentProfile(token);
                context = {
                    token,
                    profile,
                    updatedOn: new Date().toISOString()
                };
                await setAppContext(context);
            }

            if(action === "loginWindows") {
                const token = await loginWithWindows();
                profile = await getCurrentProfile(token);
                context = {
                    token,
                    profile,
                    updatedOn: new Date().toISOString()
                };
                await setAppContext(context);
            }

            if(!profile) {
                goto('/profile');
            }
            
            
        } catch (err) {
            await error("Login failed: " + err);
        }
    };

</script>

<main class="flex h-screen w-full justify-center items-center">
    <div class="flex flex-col items-center">
        <div class="mb-20">
            <div class="flex justify-center items-center">
                <img
                    data-tauri-drag-region
                    class="brightness-75 px-4"
                    alt="logo"
                    src="64x64.png"
                />
                <div data-tauri-drag-region class="text-5xl font-cascadia-mono text-gray-300 font-bold cursor-none">{appName}</div>
            </div>
        </div>
        <div class="flex flex-col space-y-4 w-80">
            <div class="flex flex-col">
                <input
                    class="border px-2 py-2 rounded w-full bg-transparent"
                    type="text"
                    placeholder="Username"
                    bind:value={state.username}
                />
            </div>

            <div class="flex flex-col">
                <input
                    class="border px-2 py-2 rounded w-full bg-transparent"
                    type="password"
                    placeholder="Password"
                    bind:value={state.password}
                />
            </div>

            <div class="flex flex-col">
                <button
                    data-action="login"
                    class="bg-blue-600 hover:bg-blue-600/50 text-white px-4 py-2 rounded disabled:opacity-50"
                    type="button"
                    disabled={!isValid}
                    onclick={onSubmit}
                >
                    Login
                </button>
            </div>

            <div class="flex flex-col">
                <button
                    data-action="loginWindows"
                    class="bg-gray-600 hover:bg-gray-600/50  text-white px-4 py-2 rounded"
                    type="button"
                    onclick={onSubmit}
                >
                    Login with Windows
                </button>
            </div>
        </div>
    </div>
</main>