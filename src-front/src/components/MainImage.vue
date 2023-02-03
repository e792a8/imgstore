<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'
import ActionSelector from './ActionSelector.vue';
import { NImage, NThing, NGrid, NGridItem } from 'naive-ui';

defineProps(['info', 'action'])
const v_emit = defineEmits(['update:action'])

const options = [
  "K", "C", "M", "D"
]

function update_action(e: any) {
  v_emit('update:action', e)
}

</script>

<template>
  <n-image :src="convertFileSrc(info.path)" object-fit="scale-down" width="180" height="180" />
  <n-thing>
    <template #header>{{ info.name }}</template>
    <table>
      <tbody>
        <tr>
          <th>Size</th>
          <td>{{ info.size.toLocaleString() }}</td>
        </tr>
        <tr>
          <th>Resolution</th>
          <td>{{ info.res[0] }}x{{ info.res[1] }}</td>
        </tr>
      </tbody>
    </table>
    <template #action>
      <ActionSelector :options="options" :value="action" @update:value="update_action" />
    </template>
  </n-thing>
</template>

<style scoped>
th {
  text-align: right;
}

td {
  font-weight: bold;
}
</style>
