<script setup lang="ts">
import { NResult, NButton } from 'naive-ui';
import { appWindow } from '@tauri-apps/api/window';

</script>

<template>
  <n-result status="success" title="Done">
    <template #footer>
      <n-button @click="appWindow.close()">Close</n-button>
    </template>
  </n-result>
</template>
