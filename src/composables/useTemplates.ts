import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface Template {
  name: string;
  source: "bundled" | "user";
}

export function useTemplates() {
  const templates = ref<Template[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const userTemplatesDir = ref<string | null>(null);

  // Fetch all available template names with source info
  async function refreshTemplates() {
    loading.value = true;
    error.value = null;
    try {
      // Get user templates dir for reference
      userTemplatesDir.value = await invoke<string>("get_user_templates_dir_cmd");

      // Get all templates (now returns name + source directly)
      const result: Template[] = await invoke<Template[]>("get_templates");
      templates.value = result;
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  // Get template content for editing
  async function getTemplateContent(name: string): Promise<string> {
    return invoke<string>("get_template_content", { templateName: name });
  }

  // Save/create user template
  // Returns error message if trying to modify built-in
  async function saveUserTemplate(name: string, content: string): Promise<{ success: boolean; error?: string }> {
    try {
      await invoke("save_user_template", { templateName: name, content });
      await refreshTemplates();
      return { success: true };
    } catch (e) {
      return { success: false, error: String(e) };
    }
  }

  // Delete user template
  // Returns error message if trying to delete built-in
  async function deleteUserTemplate(name: string): Promise<{ success: boolean; error?: string }> {
    try {
      await invoke("delete_user_template", { templateName: name });
      await refreshTemplates();
      return { success: true };
    } catch (e) {
      return { success: false, error: String(e) };
    }
  }

  // Export template to file path
  async function exportTemplate(name: string, destination: string): Promise<{ success: boolean; error?: string }> {
    try {
      await invoke("export_template", { templateName: name, destination });
      return { success: true };
    } catch (e) {
      return { success: false, error: String(e) };
    }
  }

  // Import template from file path
  // Returns imported template name, or error
  async function importTemplate(sourcePath: string): Promise<{ success: boolean; name?: string; error?: string }> {
    try {
      const name: string = await invoke("import_template", { sourcePath });
      await refreshTemplates();
      return { success: true, name };
    } catch (e) {
      return { success: false, error: String(e) };
    }
  }

  // Get user templates directory path
  async function getUserTemplatesDir(): Promise<string> {
    return invoke<string>("get_user_templates_dir_cmd");
  }

  return {
    templates,
    loading,
    error,
    userTemplatesDir,
    refreshTemplates,
    getTemplateContent,
    saveUserTemplate,
    deleteUserTemplate,
    exportTemplate,
    importTemplate,
    getUserTemplatesDir,
  };
}
