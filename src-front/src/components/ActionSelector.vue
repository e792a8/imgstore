<script setup lang="ts">
import { ref } from 'vue'
import { NButton, NButtonGroup, NSpace } from 'naive-ui';

const props = defineProps(['options', 'value'])
const v_emit = defineEmits(['update:value'])

const value_ref = ref(props.value)

function update(e: any) {
  var tg = e.target
  while (tg.parentElement && !tg.value) {
    tg = tg.parentElement
  }
  value_ref.value = tg.value
  v_emit('update:value', tg.value)
}

const btn_def: any = {
  "K": {
    text: "Keep",
    color: "#18a058"
  },
  "C": {
    text: "Copy",
    color: "#2080f0"
  },
  "M": {
    text: "Move",
    color: "#f0a020"
  },
  "D": {
    text: "Delete",
    color: "#d03050"
  }
}
</script>

<template>
  <n-button-group>
    <n-button v-for="opt in options" strong type="primary" :tertiary="value_ref != opt" :color="btn_def[opt].color"
      :value="opt" @click="update">{{
  btn_def[opt].text
      }}</n-button>
  </n-button-group>
</template>

<style scoped>

</style>