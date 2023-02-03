<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'
import ActionSelector from './ActionSelector.vue'
import { NImage, NListItem, NThing, NRow, NTable } from 'naive-ui';

const props = defineProps(['info', 'main_info', 'action', 'idx'])
const v_emit = defineEmits(['update:action'])

function compsel(a: any, b: any, gt: any, lt: any, eq: any) {
  return a > b ? gt : a < b ? lt : eq
}

const name_cls = props.info.name == props.main_info.name ? 'fname_same' : ''

const size_cls = compsel(props.info.size, props.main_info.size, 'size_gt', 'size_lt', '')

const res_cls = compsel(props.info.res[0] * props.info.res[1], props.main_info.res[0] * props.main_info.res[1], 'res_gt', 'res_lt', '')

const options = [
  "K", "D"
]

function update_action(e: any) {
  v_emit('update:action', e)
}

</script>

<template>
  <n-list-item>
    <template #prefix>
      <n-image :src="convertFileSrc(info.path)" object-fit="scale-down" width="180" height="180" />
    </template>
    <n-thing>
      <template #header><span :class="name_cls">{{ info.name }}</span></template>
      <table>
        <tr>
          <th>Size</th>
          <td :class="size_cls">{{ info.size.toLocaleString() }}</td>
        </tr>
        <tr>
          <th>Resolution</th>
          <td :class="res_cls">{{ info.res[0] }}x{{ info.res[1] }}</td>
        </tr>
        <tr>
          <th>Similarity</th>
          <td>{{ info.sim.toFixed(3) }}</td>
        </tr>
      </table>
      <template #action>
        <n-row>
          <ActionSelector :options="options" :value="action" @update:value="update_action" />
        </n-row>
      </template>
    </n-thing>
  </n-list-item>
</template>

<style scoped>
th {
  text-align: right;
}

td {
  font-weight: bold;
}

.fname_same {
  color: tomato;
}

.size_gt {
  color: brown;
}

.size_lt {
  color: cadetblue;
}

.res_gt {
  color: limegreen;
}

.res_lt {
  color: crimson;
}
</style>
