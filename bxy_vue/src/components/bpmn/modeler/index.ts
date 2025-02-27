import inherits from "inherits"

import Modeler from "bpmn-js/lib/Modeler"

import CustomModule from "./custom"

function CustomModeler(options) {
  Modeler.call(this, options)

  this._customElements = []
}

inherits(CustomModeler, Modeler)

CustomModeler.prototype._modules = [].concat(CustomModeler.prototype._modules, [
  CustomModule
])

export { CustomModeler }

