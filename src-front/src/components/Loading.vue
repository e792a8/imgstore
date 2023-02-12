<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { ref } from 'vue';
import { NProgress, NSpin } from 'naive-ui';

const state = ref([0, 0])

listen<[number, number]>('progress', (ev) => {
  state.value = ev.payload
})

</script>

<template>
  <n-spin size="large" :show="state[1] == 0">
    <div v-if="state[1] != 0">
      <n-progress type="line" :show-indicator="false" :processing="state[1] == 0"
        :percentage="state[0] * 100 / state[1]" />
      <p style="text-align: center;">{{ state[0] }} / {{ state[1] }}</p>
    </div>
  </n-spin>
</template>
