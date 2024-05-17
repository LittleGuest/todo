<script setup>
import { onMounted, ref, h } from "vue";
import { useRouter, useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";

import Project from "./Project.vue";
import AddProject from "./AddProject.vue";

const router = useRouter();
const route = useRoute();

// 项目列表
const projects = ref([]);
const addLayerIndex = ref();

async function projectsApi() {
  projects.value = await invoke("projects", { keyword: null });
}

onMounted(() => {
  projectsApi();
});

const openAddProjectForm = () => {
  addLayerIndex.value = layer.open({
    type: "page",
    title: "添加项目",
    content: h(AddProject, { addLayerIndex: addLayerIndex.value }),
  });
};
</script>

<template>
  <lay-button
    prefix-icon="layui-icon-add-circle"
    suffix-icon="layui-icon-add-circle"
    @click="openAddProjectForm"
    >添加项目</lay-button
  >

  <Project v-for="p in projects" :name="p.name" />
</template>

<style scoped></style>
