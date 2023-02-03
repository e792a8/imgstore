<script setup lang="ts">
import Loading from './components/Loading.vue'
import MainBody from './components/MainBody.vue'
import Done from './components/Done.vue'
import { ref } from 'vue'
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

export interface Args {
  storedir: String,
  filepath: String,
  sim_th: number,
}

export interface MainImgInfo {
  path: String,
  name: String,
  size: number,
  res: [number, number],
}

export interface ImgElemInfo {
  path: String,
  name: String,
  size: number,
  res: [number, number],
  sim: number,
}

export type ImgsInfo = [MainImgInfo, Array<ImgElemInfo>]

export type Answer = [String, Array<String>]

const status = ref("loading")

listen<string>('status', (ev) => {
  status.value = ev.payload
})

const imgs = ref<ImgsInfo>()

const args = ref<Args>()

invoke<Args>("get_cmd_args").then(a => {
  args.value = a
  invoke<ImgsInfo>("process", {
    args: a
  }).then(v => {
    imgs.value = v
    status.value = "main"
  })
})

function main_done() {
  status.value = 'done'
}

</script>

<template>
  <main>
    <Loading v-if="status == 'loading'" />
    <MainBody v-if="status == 'main'" :args="args" :imgs="imgs" @main_done="main_done" />
    <Done v-if="status == 'done'" />
  </main>
</template>

<style scoped>

</style>
