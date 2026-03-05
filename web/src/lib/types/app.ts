export interface Project {
    id: string;
    name: string;
    updatedOn: string;
}

export interface LoadResult {
    sessionId: string;
}

export interface LoginArgs {
    username: string;
    password: string;
}

export interface UserProfile {

}

export interface Context {
    token: string;
    updatedOn: string;
    profile: UserProfile | null;
}

export interface UserProfile {
    
}

export type UpdateStatus = { type: "checking" } 
    | { type: "found", version: string }
    | { type: "latest", version: string }
    | { type: "downloading", version: string, downloaded: number, fileSize: number }
    | { type: "downloaded", version: string };