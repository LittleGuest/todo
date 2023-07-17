import { createRouter, createWebHashHistory } from "vue-router";

import ProjectList from "./components/project/ProjectList.vue";
import AddProject from "./components/project/AddProject.vue";

import TaskList from "./components/task/TaskList.vue";
import AddTask from "./components/task/AddTask.vue";

const routes = [
  { path: "/", component: ProjectList },
  { path: "/task", component: TaskList },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
