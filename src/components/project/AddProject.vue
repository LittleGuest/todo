<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

const projectName = ref("");
const add = () => {
  console.log(projectName.value, "projectName");
};
</script>

<template>
  <lay-form style="padding: 15px">
    <lay-input
      v-model="projectName"
      placeholder="请输入项目名称"
      :allow-clear="true"
      :maxlength="100"
    ></lay-input>
    <lay-button type="primary" @click="add">添加</lay-button>
  </lay-form>
</template>
