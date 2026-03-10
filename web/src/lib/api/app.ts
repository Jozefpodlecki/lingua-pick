import type { Context, CreateProfileArgs, Language, LanguageFeature, LoadResult, LoginArgs, Project, Session, UpdateStatus, UserProfile } from "$lib/types/app";
import { invoke } from "@tauri-apps/api/core";
import { emit, listen, type EventCallback, type UnlistenFn } from "@tauri-apps/api/event";

export const load = (): Promise<LoadResult> => invoke("load_app");

export const checkUpdates = async (handler: (event: { payload: UpdateStatus }) => void): Promise<void> => {
    await listen("on-update", handler);

    await emit("check_updates");
}

export const install = (): Promise<void> => emit("install");

export const loginWithCreds = (args: LoginArgs & Record<string, unknown>): Promise<string> => invoke("login_with_creds", args);

export const loginWithWindows = (): Promise<string> => invoke("login_with_windows");

export const getCurrentProfile = (token: string): Promise<UserProfile | null> => invoke("get_current_profile", { token });

export const setAppContext = (context: Context): Promise<void> => {
    const json = JSON.stringify(context);
    localStorage.setItem("context", json);

    return Promise.resolve()
}

export const getAppContext = (): Promise<Context | null> => {

    const context = JSON.parse(localStorage.getItem("context") || null as any) || null;

    return Promise.resolve(context)
}

export const createProfile = (args: CreateProfileArgs): Promise<void> => emit("create-profile", args);

export const getDefaultLanguage = (token: string): Promise<Language | null> => invoke("get_default_language");

export const getAllLanguages = (token: string): Promise<Language[]> => invoke("get_all_languages", { token });

export const getLanguageFeatures = (token: string, id: number): Promise<LanguageFeature[]> => invoke("get_language_features");

export const getSession = (token: string, id: string): Promise<Session> => invoke("get_session");