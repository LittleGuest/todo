<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";

const router = useRouter();
const route = useRoute();

const props = defineProps({
  name: String,
});

const toTask = () => {
  router.push("/task");
};
</script>

<template>
  <lay-panel @click="toTask">{{ name }}</lay-panel>
</template>

<style scoped></style>
