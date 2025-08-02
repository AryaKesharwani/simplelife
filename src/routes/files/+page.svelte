<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface LocalFile {
    name: string;
    path: string;
    size: number;
    type: string;
  }

  interface UploadedFile {
    id?: number;
    name: string;
    path: string;
    size: number;
    type: string;
    uploaded_at: string;
    status: string;
  }

  let selectedFiles = $state<LocalFile[]>([]);
  let uploadedFiles = $state<UploadedFile[]>([]);
  let isLoading = $state(true);
  let isUploading = $state(false);
  let uploadProgress = $state(0);
  let uploadStatus = $state("");
  let searchTerm = $state("");
  let selectedFileType = $state("all");

  const fileTypes = [
    { id: "all", name: "All Files", icon: "📁" },
    { id: "image", name: "Images", icon: "🖼️" },
    { id: "document", name: "Documents", icon: "📄" },
    { id: "video", name: "Videos", icon: "🎥" },
    { id: "audio", name: "Audio", icon: "🎵" },
    { id: "archive", name: "Archives", icon: "📦" }
  ];

  onMount(async () => {
    await loadUploadedFiles();
  });

  async function loadUploadedFiles() {
    try {
      isLoading = true;
      // For now, we'll simulate uploaded files
      uploadedFiles = [
        {
          id: 1,
          name: "document.pdf",
          path: "/uploads/document.pdf",
          size: 1024000,
          type: "document",
          uploaded_at: new Date().toISOString(),
          status: "uploaded"
        },
        {
          id: 2,
          name: "image.jpg",
          path: "/uploads/image.jpg",
          size: 2048000,
          type: "image",
          uploaded_at: new Date().toISOString(),
          status: "uploaded"
        }
      ];
    } catch (error) {
      console.error('Error loading uploaded files:', error);
    } finally {
      isLoading = false;
    }
  }

  async function selectFiles() {
    try {
      const result = await invoke('open_file_dialog') as string[];
      if (result && result.length > 0) {
        const newFiles: LocalFile[] = result.map(path => {
          const name = path.split('/').pop() || path.split('\\').pop() || 'Unknown';
          return {
            name,
            path,
            size: 0, // We'll get this from the file system
            type: getFileType(name)
          };
        });
        selectedFiles = [...selectedFiles, ...newFiles];
      }
    } catch (error) {
      console.error('Error selecting files:', error);
    }
  }

  async function uploadToS3() {
    if (selectedFiles.length === 0) return;

    try {
      isUploading = true;
      uploadProgress = 0;
      uploadStatus = "Starting upload...";

      for (let i = 0; i < selectedFiles.length; i++) {
        const file = selectedFiles[i];
        uploadStatus = `Uploading ${file.name}...`;
        uploadProgress = ((i + 1) / selectedFiles.length) * 100;

        // Simulate upload delay
        await new Promise(resolve => setTimeout(resolve, 1000));

        // For now, we'll simulate the upload
        await invoke('upload_to_s3', { 
          request: { 
            file_path: file.path,
            bucket: "my-bucket",
            region: "us-east-1"
          } 
        });
      }

      uploadStatus = "Upload completed successfully!";
      selectedFiles = [];
      
      // Reload uploaded files
      await loadUploadedFiles();
    } catch (error) {
      console.error('Error uploading files:', error);
      uploadStatus = `Upload failed: ${error}`;
    } finally {
      isUploading = false;
    }
  }

  function removeFile(index: number) {
    selectedFiles = selectedFiles.filter((_, i) => i !== index);
  }

  function clearAll() {
    selectedFiles = [];
    uploadProgress = 0;
    uploadStatus = "";
  }

  function getFileType(filename: string): string {
    const ext = filename.split('.').pop()?.toLowerCase();
    if (['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp'].includes(ext || '')) return 'image';
    if (['pdf', 'doc', 'docx', 'txt', 'rtf'].includes(ext || '')) return 'document';
    if (['mp4', 'avi', 'mov', 'wmv', 'flv'].includes(ext || '')) return 'video';
    if (['mp3', 'wav', 'flac', 'aac'].includes(ext || '')) return 'audio';
    if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext || '')) return 'archive';
    return 'other';
  }

  function formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  function getFilteredFiles() {
    let filtered = uploadedFiles;
    
    if (searchTerm) {
      filtered = filtered.filter(file => 
        file.name.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
    
    if (selectedFileType !== "all") {
      filtered = filtered.filter(file => file.type === selectedFileType);
    }
    
    return filtered;
  }

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  function getFileIcon(type: string) {
    switch (type) {
      case 'image': return '🖼️';
      case 'document': return '📄';
      case 'video': return '🎥';
      case 'audio': return '🎵';
      case 'archive': return '📦';
      default: return '📁';
    }
  }

  function getFileTypeColor(type: string) {
    switch (type) {
      case 'image': return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400';
      case 'document': return 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400';
      case 'video': return 'bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-400';
      case 'audio': return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400';
      case 'archive': return 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-400';
      default: return 'bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400';
    }
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-50 dark:from-gray-950 dark:via-gray-900 dark:to-gray-950">
  <!-- Header -->
  <header class="bg-white/80 dark:bg-gray-800/80 backdrop-blur-md border-b border-gray-200 dark:border-gray-700 sticky top-0 z-40">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between py-4">
        <div class="flex items-center space-x-4">
          <a href="/" class="flex items-center space-x-2 text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
            </svg>
            <span>Back to Hub</span>
          </a>
        </div>
        <div class="flex items-center space-x-4">
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">File Manager</h1>
          <button
            onclick={selectFiles}
            class="flex items-center space-x-2 px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            <span>Select Files</span>
          </button>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    {#if isLoading}
      <div class="flex items-center justify-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-green-600"></div>
      </div>
    {:else}
      <!-- Upload Section -->
      {#if selectedFiles.length > 0}
        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6 mb-8">
          <div class="flex items-center justify-between mb-4">
            <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
              Files to Upload ({selectedFiles.length})
            </h2>
            <div class="flex items-center space-x-2">
              <button
                onclick={clearAll}
                class="px-3 py-1 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
              >
                Clear All
              </button>
              <button
                onclick={uploadToS3}
                disabled={isUploading}
                class="px-4 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-400 text-white rounded-lg transition-colors duration-200"
              >
                {isUploading ? 'Uploading...' : 'Upload to S3'}
              </button>
            </div>
          </div>

          {#if isUploading}
            <div class="mb-4">
              <div class="flex items-center justify-between text-sm text-gray-600 dark:text-gray-400 mb-2">
                <span>{uploadStatus}</span>
                <span>{uploadProgress.toFixed(0)}%</span>
              </div>
              <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                <div class="bg-green-600 h-2 rounded-full transition-all duration-300" style="width: {uploadProgress}%"></div>
              </div>
            </div>
          {/if}

          <div class="space-y-2">
            {#each selectedFiles as file, index}
              <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
                <div class="flex items-center space-x-3">
                  <span class="text-2xl">{getFileIcon(getFileType(file.name))}</span>
                  <div>
                    <p class="text-sm font-medium text-gray-900 dark:text-white">{file.name}</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">{formatFileSize(file.size)}</p>
                  </div>
                </div>
                <button
                  onclick={() => removeFile(index)}
                  class="p-1 text-gray-400 hover:text-red-600 transition-colors"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                  </svg>
                </button>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Search and Filters -->
      <div class="mb-8 space-y-4">
        <div class="flex flex-col sm:flex-row gap-4">
          <div class="flex-1">
            <div class="relative">
              <svg class="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
              </svg>
              <input
                type="text"
                bind:value={searchTerm}
                placeholder="Search files..."
                class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
              />
            </div>
          </div>
          <div class="flex space-x-2">
            {#each fileTypes as fileType}
              <button
                onclick={() => selectedFileType = fileType.id}
                class="px-4 py-2 rounded-lg transition-all duration-200 {selectedFileType === fileType.id 
                  ? 'bg-green-600 text-white shadow-md' 
                  : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              >
                <span class="mr-2">{fileType.icon}</span>
                {fileType.name}
              </button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Uploaded Files Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each getFilteredFiles() as file, index}
          <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all duration-200 group">
            <div class="p-6">
              <div class="flex items-start justify-between mb-4">
                <div class="flex items-center space-x-2">
                  <span class="text-3xl">{getFileIcon(file.type)}</span>
                  <span class="px-2 py-1 text-xs font-medium rounded-full {getFileTypeColor(file.type)}">
                    {file.type.charAt(0).toUpperCase() + file.type.slice(1)}
                  </span>
                </div>
                <div class="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                  <button
                    onclick={() => navigator.clipboard.writeText(file.path)}
                    class="p-1 text-gray-400 hover:text-green-600 transition-colors"
                    title="Copy path"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                    </svg>
                  </button>
                </div>
              </div>
              
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2 line-clamp-2">
                {file.name}
              </h3>
              
              <div class="space-y-2 text-sm text-gray-600 dark:text-gray-400">
                <div class="flex justify-between">
                  <span>Size:</span>
                  <span class="font-medium">{formatFileSize(file.size)}</span>
                </div>
                <div class="flex justify-between">
                  <span>Uploaded:</span>
                  <span class="font-medium">{formatDate(file.uploaded_at)}</span>
                </div>
                <div class="flex justify-between">
                  <span>Status:</span>
                  <span class="font-medium text-green-600 dark:text-green-400">{file.status}</span>
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>

      {#if getFilteredFiles().length === 0}
        <div class="text-center py-16">
          <div class="text-6xl mb-4">📁</div>
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            {searchTerm || selectedFileType !== "all" ? "No files found" : "No uploaded files yet"}
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {searchTerm || selectedFileType !== "all" 
              ? "Try adjusting your search or filters" 
              : "Select files and upload them to get started"}
          </p>
          {#if !searchTerm && selectedFileType === "all"}
            <button
              onclick={selectFiles}
              class="inline-flex items-center space-x-2 px-6 py-3 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors duration-200"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
              </svg>
              <span>Select Files</span>
            </button>
          {/if}
        </div>
      {/if}
    {/if}
  </main>
</div> 