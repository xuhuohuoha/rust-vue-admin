import { defineStore } from "pinia"

export const useBpmnStore = defineStore(
  'bpmnStore',
  () => {
    // Bpmn Modeler
    const modeler = ref()

    const setModeler = (modeler) => {
      modeler.value = modeler
    }

    const newBpmn = () => {

    }

    const openBpmn = () => {
    }

    const saveBpmn = () => {

    }

    const deleteBpmn = () => {

    }

    const save2Xml = () => {
    }

    const save2Image = () => {

    }

    const preview = () => { }

    const mock = () => { }

    const undo = () => { }

    const redo = () => { }

    const clean = () => { }

    const zoom = (radio: number) => { }

    return {}
  }
)
