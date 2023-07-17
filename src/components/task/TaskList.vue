<script setup>
import { ref, h } from "vue";
import { useRouter, useRoute } from "vue-router";

import AddTask from "./AddTask.vue";

const router = useRouter();
const route = useRoute();

const taskList = ref([
  {
    id: 1,
    projectId: 1,
    title: "任务1",
    desc: "描述",
    priority: "Urgency",
    tags: ["标签1", "标签2", "标签3"],
    startTime: 1689346447906,
    endTime: 1689346447906,
    status: "InProgress",
    tasks: [
      {
        id: 11,
        /// 任务标题
        title: "子任务1",
        /// 开始时间
        startTime: 1689346491919,
        /// 结束时间
        endTime: 1689346491919,
        /// 状态
        status: "NotStarted",

        createAt: 1689346491919,
        updateAt: 1689346491919,
        deleteAt: 1689346491919,
      },
      {
        id: 12,
        /// 任务标题
        title:
          "子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务子任务2",
        /// 开始时间
        startTime: 1689346491919,
        /// 结束时间
        endTime: 1689346491919,
        /// 状态
        status: "NotStarted",

        createAt: 1689346491919,
        updateAt: 1689346491919,
        deleteAt: 1689346491919,
      },
    ],
    createAt: 1689346447906,
    updateAt: 1689346447906,
    deleteAt: 1689346447906,
  },
  {
    id: 2,
    title:
      "任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务任务2",
    desc: "描述",
    priority: "High",
    tags: ["标签1", "标签2", "标签3"],
    startTime: 1689346447906,
    endTime: 1689346447906,
    status: "NotStarted",
    tasks: [],
    createAt: 1689346447906,
    updateAt: 1689346447906,
    deleteAt: 1689346447906,
  },
]);

const openAddProjectForm = () => {
  layer.open({
    type: "page",
    title: "添加任务",
    content: h(AddTask),
  });
};

const loading = ref(false);
// 选中的行
const selectedColumns = ref([]);
// const page = ref({ current: 1, limit: 10, total: 100 });

// 表格标题定义
const columns = ref([
  { title: "", width: "6px", type: "checkbox", fixed: "left", key: "id" },
  { title: "任务", width: "80px", key: "title" },
  { title: "优先级", width: "8px", key: "priority", customSlot: "priority" },
  { title: "状态", width: "10px", key: "status", customSlot: "status" },
  {
    title: "操作",
    width: "8px",
    customSlot: "operator",
    key: "operator",
    fixed: "right",
  },
]);

const change = (page) => {
  loading.value = true;
  setTimeout(() => {
    // dataSource.value = loadDataSource(page.current, page.limit);
    loading.value = false;
  }, 1000);
};

const sortChange = (key, sort) => {
  layer.msg(`字段${key} - 排序${sort}, 你可以利用 sort-change 实现服务端排序`);
};

// 删除选中的任务
const remove = () => {
  layer.msg(selectedColumns.value, { area: "50%" });
};

// const loadDataSource = (page, pageSize) => {
//   var response = [];
//   var startIndex = (page - 1) * pageSize + 1;
//   var endIndex = page * pageSize;
//   for (var i = startIndex; i <= endIndex; i++) {
//     response.push({
//       id: `${i}`,
//       age: "18",
//       sex: "男",
//       name: `张三${i}`,
//       email: "test@qq.com",
//       remark: "花开堪折直须折,莫待无花空折枝.",
//       joinTime: "2022-02-09",
//       city: "浙江杭州",
//       status: true,
//     });
//   }
//   return response;
// };

const back = () => {
  router.back();
};
</script>

<template>
  <lay-button size="xs" @click="back">回退</lay-button>
  <!-- skin="line" -->
  <lay-table
    size="sm"
    :columns="columns"
    :loading="loading"
    :default-toolbar="['filter', 'export']"
    :data-source="taskList"
    childrenColumnName="tasks"
    v-model:selected-keys="selectedColumns"
    @change="change"
    @sortChange="sortChange"
  >
    <template v-slot:toolbar>
      <lay-button
        type="primary"
        size="sm"
        prefix-icon="layui-icon-add-circle"
        suffix-icon="layui-icon-add-circle"
        @click="openAddProjectForm"
        >添加任务</lay-button
      >
      <lay-button size="sm" @click="remove">删除</lay-button>
    </template>

    <template v-slot:priority="{ row }">
      <lay-select v-model="row.priority">
        <lay-select-option value="Urgency" label="急"></lay-select-option>
        <lay-select-option value="High" label="高"></lay-select-option>
        <lay-select-option value="Mid" label="中"></lay-select-option>
        <lay-select-option value="Low" label="低"></lay-select-option>
      </lay-select>
    </template>

    <template v-slot:status="{ row }">
      <lay-select v-model="row.status">
        <lay-select-option
          value="NotStarted"
          label="未开始"
        ></lay-select-option>
        <lay-select-option
          value="InProgress"
          label="进行中"
        ></lay-select-option>
        <lay-select-option value="Delayed" label="已延期"></lay-select-option>
        <lay-select-option value="Ended" label="已完成"></lay-select-option>
        <lay-select-option value="Expired" label="已失效"></lay-select-option>
      </lay-select>
    </template>

    <template v-slot:operator="{ row }">
      <lay-button size="xs" @click="openAddProjectForm">详情</lay-button>
    </template>
  </lay-table>
</template>

<style scoped></style>
