<script lang="ts">
    import { setAppContext, loginWithCreds, loginWithWindows, getCurrentProfile } from "$lib/api/app";
    import type { Context } from "$lib/types/app";
    import type { MouseEventHandler } from "svelte/elements";

    let state = $state({
        username: "",
        password: ""
    });

    const isValid = $derived(state.username.trim() !== "" && state.password.trim() !== "");

    const onSubmit: MouseEventHandler<HTMLElement> = async (event) => {
        const { login, loginWindows } = event.currentTarget.dataset;

        try {
            if(login) {
                const token = await loginWithCreds(state);
                const profile = await getCurrentProfile(token);
                const context: Context = {
                    token,
                    profile,
                    updatedOn: new Date().toISOString()
                };
                await setAppContext(context);
            }

            if(loginWindows) {
                const token = await loginWithWindows();
                const profile = await getCurrentProfile(token);
                const context: Context = {
                    token,
                    profile,
                    updatedOn: new Date().toISOString()
                };
                await setAppContext(context);
            }
        } catch (error) {
            console.error("Login failed:", error);
        }
    };

</script>

<main class="flex h-screen w-full justify-center items-center">
    <div class="flex flex-col space-y-4 w-80">
        <div class="flex flex-col">
            <input
                class="border px-2 py-1 rounded w-full"
                type="text"
                placeholder="Username"
                bind:value={state.username}
            />
        </div>

        <div class="flex flex-col">
            <input
                class="border px-2 py-1 rounded w-full"
                type="password"
                placeholder="Password"
                bind:value={state.password}
            />
        </div>

        <div class="flex flex-col">
            <button
                data-action="login"
                class="bg-blue-600 text-white px-4 py-2 rounded disabled:opacity-50"
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
                class="bg-gray-600 text-white px-4 py-2 rounded"
                type="button"
                onclick={onSubmit}
            >
                Login with Windows
            </button>
        </div>
    </div>
</main>