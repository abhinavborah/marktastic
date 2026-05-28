<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useTemplates, type Template } from "../composables/useTemplates";
import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
import { useToast } from "../composables/useToast";

const {
  templates,
  loading,
  error,
  refreshTemplates,
  getTemplateContent,
  deleteUserTemplate,
  importTemplate,
  exportTemplate,
  saveUserTemplate,
} = useTemplates();

const toast = useToast();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const showEditor = ref(false);
const editingTemplate = ref<Template | null>(null);
const editingContent = ref("");
const showDeleteConfirm = ref(false);
const templateToDelete = ref<string | null>(null);
const isSaving = ref(false);

onMounted(() => {
  refreshTemplates();
});

async function handleImport() {
  try {
    const file = await openDialog({
      filters: [{ name: "Typst Templates", extensions: ["typ"] }],
      multiple: false,
    });
    if (file) {
      const result = await importTemplate(file as string);
      if (result.success) {
        toast.success(`Imported template: ${result.name}`);
      } else {
        toast.error(result.error || "Import failed");
      }
    }
  } catch (e) {
    toast.error(`Import failed: ${e}`);
  }
}

async function handleExport(template: Template) {
  try {
    const dest = await saveDialog({
      defaultPath: `${template.name}.typ`,
      filters: [{ name: "Typst Templates", extensions: ["typ"] }],
    });
    if (dest) {
      const result = await exportTemplate(template.name, dest);
      if (result.success) {
        toast.success(`Exported to ${dest.split(/[/\\]/).pop()}`);
      } else {
        toast.error(result.error || "Export failed");
      }
    }
  } catch (e) {
    toast.error(`Export failed: ${e}`);
  }
}

async function handleEdit(template: Template) {
  try {
    const content = await getTemplateContent(template.name);
    editingTemplate.value = template;
    editingContent.value = content;
    showEditor.value = true;
  } catch (e) {
    toast.error(`Failed to load template: ${e}`);
  }
}

async function handleSave() {
  if (!editingTemplate.value || !editingTemplate.value.name) {
    toast.error("Template name is required");
    return;
  }
  isSaving.value = true;
  try {
    const result = await saveUserTemplate(editingTemplate.value.name, editingContent.value);
    if (result.success) {
      toast.success(`Saved template: ${editingTemplate.value.name}`);
      handleEditorClose();
    } else {
      toast.error(result.error || "Save failed");
    }
  } catch (e) {
    toast.error(`Save failed: ${e}`);
  } finally {
    isSaving.value = false;
  }
}

async function handleDelete(name: string) {
  templateToDelete.value = name;
  showDeleteConfirm.value = true;
}

async function confirmDelete() {
  if (templateToDelete.value) {
    const result = await deleteUserTemplate(templateToDelete.value);
    if (result.success) {
      toast.success(`Deleted template: ${templateToDelete.value}`);
    } else {
      toast.error(result.error || "Delete failed");
    }
  }
  showDeleteConfirm.value = false;
  templateToDelete.value = null;
}

function handleEditorClose() {
  showEditor.value = false;
  editingTemplate.value = null;
  editingContent.value = "";
}

function handleClose() {
  emit("close");
}

function handleCreateNew() {
  editingTemplate.value = { name: "", source: "user" };
  editingContent.value = `// New Template
// Replace this with your Typst template content.
// Use // MARKTASTIC_BODY_CONTENT to mark where the document content will be inserted.

#show heading.where(level: 1): it => {
  text(20pt, weight: "bold", it.body)
  v(0.5em)
}

`;

  showEditor.value = true;
}
</script>

<template>
  <div class="h-full flex flex-col bg-background text-foreground">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b">
      <h2 class="text-lg font-semibold">Manage Templates</h2>
      <div class="flex items-center gap-2">
        <button
          class="p-1.5 rounded hover:bg-muted transition-colors"
          title="Close"
          @click="handleClose"
        >
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
        <button
          class="inline-flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium rounded-md bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
          @click="handleCreateNew"
        >
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 5v14M5 12h14" />
          </svg>
          New
        </button>
      </div>
    </div>

    <!-- Template List -->
    <div class="flex-1 overflow-auto p-4">
      <!-- Loading -->
      <div v-if="loading" class="flex items-center justify-center py-8">
        <svg class="w-6 h-6 animate-spin text-muted-foreground" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
        </svg>
        <span class="ml-2 text-muted-foreground">Loading templates...</span>
      </div>

      <!-- Error -->
      <div v-else-if="error" class="flex items-center justify-center py-8 text-destructive">
        <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10" />
          <line x1="15" y1="9" x2="9" y2="15" />
          <line x1="9" y1="9" x2="15" y2="15" />
        </svg>
        <span class="ml-2">{{ error }}</span>
      </div>

      <!-- Empty -->
      <div v-else-if="templates.length === 0" class="flex flex-col items-center justify-center py-8 text-muted-foreground">
        <svg class="w-12 h-12 opacity-40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
          <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
          <polyline points="14 2 14 8 20 8" />
        </svg>
        <p class="mt-2 text-sm">No templates found</p>
      </div>

      <!-- Templates -->
      <div v-else class="space-y-2">
        <div
          v-for="template in templates"
          :key="template.name"
          class="flex items-center justify-between p-3 border rounded-lg bg-card hover:bg-muted/50 transition-colors"
        >
          <div class="flex items-center gap-3">
            <span class="font-medium">{{ template.name.replace(/-/g, " ") }}</span>
            <span
              v-if="template.source === 'bundled'"
              class="px-1.5 py-0.5 text-xs rounded bg-muted text-muted-foreground"
            >
              Built-in
            </span>
            <span
              v-else
              class="px-1.5 py-0.5 text-xs rounded bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400"
            >
              User
            </span>
          </div>
          <div class="flex items-center gap-1">
            <!-- Export (all templates) -->
            <button
              class="p-1.5 rounded hover:bg-muted transition-colors"
              title="Export template"
              @click="handleExport(template)"
            >
              <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
            </button>

            <!-- User template buttons -->
            <template v-if="template.source === 'user'">
              <button
                class="p-1.5 rounded hover:bg-muted transition-colors"
                title="Edit template"
                @click="handleEdit(template)"
              >
                <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
                  <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
                </svg>
              </button>
              <button
                class="p-1.5 rounded hover:bg-muted text-destructive transition-colors"
                title="Delete template"
                @click="handleDelete(template.name)"
              >
                <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="3 6 5 6 21 6" />
                  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                </svg>
              </button>
            </template>
          </div>
        </div>
      </div>
    </div>

    <!-- Footer -->
    <div class="flex items-center justify-end gap-2 px-4 py-3 border-t">
      <button
        class="inline-flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium rounded-md bg-secondary text-secondary-foreground hover:bg-secondary/90 transition-colors"
        @click="handleImport"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
          <polyline points="17 8 12 3 7 8" />
          <line x1="12" y1="3" x2="12" y2="15" />
        </svg>
        Import
      </button>
    </div>
  </div>

  <!-- Delete Confirmation Modal -->
  <Teleport to="body">
    <div
      v-if="showDeleteConfirm"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      @click.self="showDeleteConfirm = false"
    >
      <div class="bg-card border rounded-lg shadow-lg p-6 max-w-sm w-full mx-4">
        <h3 class="text-lg font-semibold mb-2">Delete Template</h3>
        <p class="text-muted-foreground mb-4">
          Are you sure you want to delete "<strong>{{ templateToDelete }}</strong>"?
          This action cannot be undone.
        </p>
        <div class="flex items-center justify-end gap-2">
          <button
            class="px-3 py-1.5 text-sm font-medium rounded-md border hover:bg-muted transition-colors"
            @click="showDeleteConfirm = false"
          >
            Cancel
          </button>
          <button
            class="px-3 py-1.5 text-sm font-medium rounded-md bg-destructive text-destructive-foreground hover:bg-destructive/90 transition-colors"
            @click="confirmDelete"
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  </Teleport>

  <!-- Editor Modal -->
  <Teleport to="body">
    <div
      v-if="showEditor"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      @click.self="handleEditorClose"
    >
      <div @click.stop class="bg-card border rounded-lg shadow-lg max-w-2xl w-full mx-4 max-h-[90vh] flex flex-col">
        <div class="flex items-center justify-between px-4 py-3 border-b">
          <h3 class="text-lg font-semibold">
            {{ editingTemplate?.name ? "Edit Template" : "New Template" }}
          </h3>
          <button
            class="p-1.5 rounded hover:bg-muted transition-colors"
            @click="handleEditorClose"
          >
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </div>

        <div class="flex items-center gap-2 px-4 py-3 border-b">
          <label class="text-sm text-muted-foreground">Name:</label>
          <input
            v-model="editingTemplate!.name"
            type="text"
            class="flex-1 px-2 py-1.5 text-sm rounded-md border bg-background focus:outline-none focus:ring-1 focus:ring-ring"
            placeholder="template-name"
            :disabled="editingTemplate?.source === 'bundled'"
          />
        </div>

        <div class="flex-1 overflow-hidden p-4">
          <textarea
            v-model="editingContent"
            class="w-full h-full min-h-[300px] px-3 py-2 text-sm font-mono rounded-md border bg-background focus:outline-none focus:ring-1 focus:ring-ring resize-none"
            placeholder="// Typst template content..."
            spellcheck="false"
          />
        </div>

        <div class="flex items-center justify-end gap-2 px-4 py-3 border-t">
          <button
            class="px-3 py-1.5 text-sm font-medium rounded-md border hover:bg-muted transition-colors"
            @click="handleEditorClose"
          >
            Cancel
          </button>
          <button
            class="px-3 py-1.5 text-sm font-medium rounded-md bg-primary text-primary-foreground hover:bg-primary/90 transition-colors disabled:opacity-50"
            :disabled="isSaving || !editingTemplate?.name"
            @click="handleSave"
          >
            {{ isSaving ? "Saving..." : "Save" }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
