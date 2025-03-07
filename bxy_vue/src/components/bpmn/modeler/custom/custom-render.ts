 
import inherits from 'inherits'

import BaseRenderer from 'diagram-js/lib/draw/BaseRenderer'

import {
  append as svgAppend,
  create as svgCreate
} from 'tiny-svg'

import { customConfig, customElements } from '../../utils/util'
/**
 * A renderer that knows how to render custom elements.
 */
export default function CustomRenderer(eventBus, styles) {
    BaseRenderer.call(this, eventBus, 2000)

    const computeStyle = styles.computeStyle

    this.drawCustomElements = function(parentNode, element) {
        console.log(element)
        const type = element.type // 获取到类型
        if (customElements.includes(type)) { // or customConfig[type]
            const { url, attr } = customConfig[type]
            const customIcon = svgCreate('image', {
                ...attr,
                href: url
            })
            element['width'] = attr.width // 这里我是取了巧, 直接修改了元素的宽高
            element['height'] = attr.height
            svgAppend(parentNode, customIcon)
            return customIcon
        }
        const shape = this.bpmnRenderer.drawShape(parentNode, element)
        return shape
    }
}

inherits(CustomRenderer, BaseRenderer)

CustomRenderer.$inject = ['eventBus', 'styles']

CustomRenderer.prototype.canRender = function(element) {
    // ignore labels
    return !element.labelTarget;
}

CustomRenderer.prototype.drawShape = function(p, element) {
    return this.drawCustomElements(p, element)
}

CustomRenderer.prototype.getShapePath = function(shape) {
    console.log(shape)
}