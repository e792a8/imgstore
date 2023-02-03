<script setup lang="ts">
import { ref } from 'vue';
import MainImage from './MainImage.vue';
import ImageElem from './ImageElem.vue';
import { invoke } from '@tauri-apps/api/tauri';
import { NList, NButton, NGrid, NGridItem, NAffix, NDivider } from 'naive-ui';

const props = defineProps(['args', 'imgs'])

const v_emit = defineEmits(['main_done'])

const img_actions = ref([''])

const confirm_loading = ref(false)

const main_img_action = ref('K')

img_actions.value = Array<string>(props.imgs[1].length).fill('K')

function confirm() {
  confirm_loading.value = true
  console.log(main_img_action.value)
  console.log(img_actions.value)
  invoke('answer', {
    args: props.args,
    imgs: props.imgs,
    ans: [main_img_action.value, img_actions.value],
  }).then(() => {
    confirm_loading.value = false
    v_emit('main_done')
  })
}

</script>

<template>
  <n-grid cols="5">
    <n-grid-item span="2">
      <n-affix :trigger-top="50">
        <MainImage v-model:action="main_img_action" :info="imgs[0]" />
        <n-divider />
        <n-button :loading="confirm_loading" type="primary" @click="confirm">Confirm</n-button>
      </n-affix>
    </n-grid-item>
    <n-grid-item span="3">
      <n-list>
        <ImageElem v-for="(el, idx) in imgs[1]" :idx="idx" v-model:action="img_actions[idx]" :info="el"
          :main_info="imgs[0]" />
      </n-list>
    </n-grid-item>
  </n-grid>
</template>
