export type ThemeMode = "light" | "dark" | "system";

export interface FileEntry {
  name: string;
  content: string;
}

export interface OpenedFolder {
  path: string;
  files: FileEntry[];
}
