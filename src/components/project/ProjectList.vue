<script setup>
import { ref, reactive, h, resolveComponent } from "vue";
import { useRouter, useRoute } from "vue-router";

import AddProject from "./AddProject.vue";

const router = useRouter();
const route = useRoute();

const projects = ref([
  {
    id: 1,
    name: "项目名称",
    status: 0,
  },
  {
    id: 2,
    name: "项目名称",
    status: 0,
  },
  {
    id: 3,
    name: "项目名称",
    status: 0,
  },
  {
    id: 4,
    name: "项目名称",
    status: 0,
  },
  {
    id: 5,
    name: "这是一个项目名称很长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长的项目",
    status: 0,
  },
]);

const openAddProjectForm = () => {
  layer.open({
    type: "page",
    title: "添加项目",
    content: h(AddProject),
  });
};

const toTask = () => {
  router.push("/task");
};
</script>

<template>
  <lay-button
    prefix-icon="layui-icon-add-circle"
    suffix-icon="layui-icon-add-circle"
    @click="openAddProjectForm"
    >添加项目</lay-button
  >

  <lay-panel v-for="p in projects" :key="p.id" @click="toTask">{{
    p.name
  }}</lay-panel>
</template>

<style scoped></style>
