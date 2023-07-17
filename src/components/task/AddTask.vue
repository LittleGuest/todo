<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

const taskForm = ref({
  id: 1,
  projectId: 1,
  title: "任务1",
  desc: "描述",
  priority: "Urgency",
  tags: ["标签1", "标签2", "标签3"],
  startTime: 1689346447906,
  endTime: 1689346447906,
  status: "NotStarted",
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
});

const projectId = ref([]);

const projectTree = ref([
  {
    title: "一级1",
    id: 1,
    field: "name1",
    children: [
      {
        title: "二级1-1 可允许跳转",
        id: 3,
        field: "name11",
        href: "https://www.layui.com/",
        children: [
          {
            title: "三级1-1-3",
            id: 23,
            field: "",
            children: [
              {
                title: "四级1-1-3-1",
                id: 24,
                field: "",
                children: [
                  {
                    title: "五级1-1-3-1-1",
                    id: 30,
                  },
                  {
                    title: "五级1-1-3-1-2",
                    id: 31,
                  },
                ],
              },
            ],
          },
          {
            title: "三级1-1-1",
            id: 7,
            field: "",
            children: [
              {
                title: "四级1-1-1-1 可允许跳转",
                id: 15,
                href: "https://www.layui.com/doc/",
              },
            ],
          },
          {
            title: "三级1-1-2",
            id: 8,
            field: "",
            children: [
              {
                title: "四级1-1-2-1",
                id: 32,
              },
            ],
          },
        ],
      },
      {
        title: "二级1-2",
        id: 4,
        spread: true,
        children: [
          {
            title: "三级1-2-1",
            id: 9,
          },
          {
            title: "三级1-2-2",
            id: 10,
          },
        ],
      },
      {
        title: "二级1-3",
        id: 20,
        field: "",
        children: [
          {
            title: "三级1-3-1",
            id: 21,
            field: "",
          },
          {
            title: "三级1-3-2",
            id: 22,
            field: "",
          },
        ],
      },
    ],
  },
  {
    title: "一级2",
    id: 2,
    children: [
      {
        title: "二级2-1",
        id: 5,
        spread: true,
        children: [
          {
            title: "三级2-1-1",
            id: 11,
          },
          {
            title: "三级2-1-2",
            id: 12,
          },
        ],
      },
    ],
  },
]);

const startEndTime = ref(["", ""]);

const add = () => {
  taskForm.value.validate((isValidate, model, errors) => {
    console.log("taskForm.validate", isValidate);
    console.log("taskForm.validate", model);
    console.log("taskForm.validate", errors);
    // layer.open({
    //   type: 1,
    //   title: "表单提交结果",
    //   content: `<div style="padding: 10px"><p>是否通过 : ${isValidate}</p> <p>表单数据 : ${JSON.stringify(
    //     model
    //   )} </p> <p>错误信息 : ${JSON.stringify(errors)}</p></div>`,
    //   shade: false,
    //   isHtmlFragment: true,
    //   btn: [
    //     {
    //       text: "确认",
    //       callback(index) {
    //         layer.close(index);
    //       },
    //     },
    //   ],
    //   area: "500px",
    // });
  });
};

const reset = () => {
  // taskForm.value.clearValidate();
  taskForm.value.reset();
};
</script>

<template>
  <lay-form :model="taskForm" ref="taskForm" required>
    <lay-form-item label="所属项目" prop="projectId">
      <lay-tree-select
        v-model="projectId"
        :data="projectTree"
        placeholder="请选择项目"
        :allow-clear="true"
        :search="true"
        :multiple="true"
      ></lay-tree-select>
    </lay-form-item>

    <lay-form-item label="任务标题" prop="title">
      <lay-input
        v-model="taskForm.title"
        placeholder="请输入任务标题"
        :allow-clear="true"
        :maxlength="100"
      ></lay-input>
    </lay-form-item>

    <lay-form-item label="起止时间" prop="startEndTime">
      <lay-date-picker
        v-model="startEndTime"
        range
        timestamp
        :placeholder="['开始日期', '结束日期']"
      ></lay-date-picker>
      <lay-date-picker v-model="taskForm.startTime" timestamp></lay-date-picker>
      <lay-date-picker v-model="taskForm.endTime" timestamp></lay-date-picker>
    </lay-form-item>

    <lay-form-item label="任务状态" prop="status">
      <lay-select v-model="taskForm.status">
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
    </lay-form-item>

    <lay-form-item label="优先级" prop="priority">
      <lay-radio v-model="taskForm.priority" name="priority" value="Urgency"
        >急</lay-radio
      >
      <lay-radio v-model="taskForm.priority" name="priority" value="High"
        >高</lay-radio
      >
      <lay-radio v-model="taskForm.priority" name="priority" value="Mid"
        >中</lay-radio
      >
      <lay-radio v-model="taskForm.priority" name="priority" value="Low"
        >低</lay-radio
      >
    </lay-form-item>

    <lay-form-item style="text-align: center">
      <lay-button type="default" @click="reset">重置</lay-button>
      <lay-button type="primary" @click="add">添加</lay-button>
    </lay-form-item>
  </lay-form>
</template>
