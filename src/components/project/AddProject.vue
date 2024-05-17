<script setup>
import { reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps({
  addLayerIndex: Number,
});

const addProjectForm = reactive({});
const addProjectValid = ref();

async function saveProjectApi() {
  await invoke("save_project", { name: addProjectForm.name });
}

const lIndex = ref();

const add = () => {
  lIndex.value = layer.load(2);
  addProjectValid.value.validate((isValidate, model, errors) => {
    if (isValidate) {
      const res = saveProjectApi();
      console.log("res", res);

      layer.notifiy({
        title: "这是标题",
        content: "默认就是右上，也是用得最多的",
      });

      layer.close(lIndex.value);
    }
  });
};
</script>

<template>
  <lay-form
    :model="addProjectForm"
    ref="addProjectValid"
    style="padding: 15px"
    required
  >
    <lay-form-item label="项目名称" prop="name">
      <lay-input
        placeholder="请输入项目名称"
        :allow-clear="true"
        :maxlength="100"
        v-model="addProjectForm.name"
        required
      ></lay-input>
    </lay-form-item>
    <lay-form-item style="text-align: center">
      <lay-button type="primary" @click="add">确定</lay-button>
    </lay-form-item>
  </lay-form>
</template>
