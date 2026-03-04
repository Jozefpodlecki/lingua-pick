import type { LoadResult, LoginArgs, Project, UpdateStatus } from "$lib/types/app";
import { invoke } from "@tauri-apps/api/core";
import { emit, listen, type EventCallback, type UnlistenFn } from "@tauri-apps/api/event";

export const load = (): Promise<LoadResult> => invoke("load_app");

export const checkUpdates = async (handler: (event: { payload: UpdateStatus }) => void): Promise<void> => {
    await listen("on-update", handler);

    await emit("check_updates");
}

export const install = (): Promise<void> => emit("install");

export const login = (args: LoginArgs & Record<string, unknown>): Promise<string> => invoke("login", args);

export const loginWithWindows = (): Promise<string> => invoke("login_with_windows");

export const getTest = (): Promise<void> => {

    localStorage.getItem("context");

    return Promise.resolve()
}