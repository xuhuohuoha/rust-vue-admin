export const bpmnXML = `
            <?xml version="1.0" encoding="UTF-8"?>
                <bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" id="sample-diagram" targetNamespace="http://bpmn.io/schema/bpmn" xsi:schemaLocation="http://www.omg.org/spec/BPMN/20100524/MODEL BPMN20.xsd">
                <bpmn2:process id="process1567044459787" name="流程1567044459787">
                    <bpmn2:documentation>描述</bpmn2:documentation>
                    <bpmn2:startEvent id="StartEvent_01ydzqe" name="开始">
                    <bpmn2:outgoing>SequenceFlow_1qw929z</bpmn2:outgoing>
                    </bpmn2:startEvent>
                    <bpmn2:sequenceFlow id="SequenceFlow_1qw929z" sourceRef="StartEvent_01ydzqe" targetRef="Task_1piqdk6" />
                    <bpmn2:userTask id="Task_1piqdk6" name="请假申请">
                    <bpmn2:incoming>SequenceFlow_1qw929z</bpmn2:incoming>
                    <bpmn2:outgoing>SequenceFlow_11h4o22</bpmn2:outgoing>
                    </bpmn2:userTask>
                    <bpmn2:exclusiveGateway id="ExclusiveGateway_0k39v3u">
                    <bpmn2:incoming>SequenceFlow_11h4o22</bpmn2:incoming>
                    <bpmn2:outgoing>SequenceFlow_1iu7pfe</bpmn2:outgoing>
                    <bpmn2:outgoing>SequenceFlow_04uqww2</bpmn2:outgoing>
                    </bpmn2:exclusiveGateway>
                    <bpmn2:sequenceFlow id="SequenceFlow_11h4o22" sourceRef="Task_1piqdk6" targetRef="ExclusiveGateway_0k39v3u" />
                    <bpmn2:sequenceFlow id="SequenceFlow_1iu7pfe" sourceRef="ExclusiveGateway_0k39v3u" targetRef="Task_10fqcwp" />
                    <bpmn2:userTask id="Task_10fqcwp" name="经理审批">
                    <bpmn2:incoming>SequenceFlow_1iu7pfe</bpmn2:incoming>
                    <bpmn2:outgoing>SequenceFlow_1xod8nh</bpmn2:outgoing>
                    </bpmn2:userTask>
                    <bpmn2:sequenceFlow id="SequenceFlow_04uqww2" sourceRef="ExclusiveGateway_0k39v3u" targetRef="Task_15n23yh" />
                    <bpmn2:userTask id="Task_15n23yh" name="总部审批">
                    <bpmn2:incoming>SequenceFlow_04uqww2</bpmn2:incoming>
                    <bpmn2:outgoing>SequenceFlow_0c8wrs4</bpmn2:outgoing>
                    </bpmn2:userTask>
                    <bpmn2:exclusiveGateway id="ExclusiveGateway_1sq33g6">
                    <bpmn2:incoming>SequenceFlow_0c8wrs4</bpmn2:incoming>
                    <bpmn2:incoming>SequenceFlow_1xod8nh</bpmn2:incoming>
                    <bpmn2:outgoing>SequenceFlow_0h8za82</bpmn2:outgoing>
                    </bpmn2:exclusiveGateway>
                    <bpmn2:sequenceFlow id="SequenceFlow_0c8wrs4" sourceRef="Task_15n23yh" targetRef="ExclusiveGateway_1sq33g6" />
                    <bpmn2:sequenceFlow id="SequenceFlow_1xod8nh" sourceRef="Task_10fqcwp" targetRef="ExclusiveGateway_1sq33g6" />
                    <bpmn2:endEvent id="EndEvent_0pnmjd3">
                    <bpmn2:incoming>SequenceFlow_0h8za82</bpmn2:incoming>
                    </bpmn2:endEvent>
                    <bpmn2:sequenceFlow id="SequenceFlow_0h8za82" sourceRef="ExclusiveGateway_1sq33g6" targetRef="EndEvent_0pnmjd3" />
                </bpmn2:process>
                <bpmndi:BPMNDiagram id="BPMNDiagram_1">
                    <bpmndi:BPMNPlane id="BPMNPlane_1" bpmnElement="process1567044459787">
                    <bpmndi:BPMNShape id="StartEvent_01ydzqe_di" bpmnElement="StartEvent_01ydzqe">
                        <dc:Bounds x="532" y="72" width="36" height="36" />
                        <bpmndi:BPMNLabel>
                        <dc:Bounds x="539" y="53" width="22" height="14" />
                        </bpmndi:BPMNLabel>
                    </bpmndi:BPMNShape>
                    <bpmndi:BPMNEdge id="SequenceFlow_1qw929z_di" bpmnElement="SequenceFlow_1qw929z">
                        <di:waypoint x="550" y="108" />
                        <di:waypoint x="550" y="150" />
                    </bpmndi:BPMNEdge>
                    <bpmndi:BPMNShape id="UserTask_1qxjy46_di" bpmnElement="Task_1piqdk6">
                        <dc:Bounds x="500" y="150" width="100" height="80" />
                    </bpmndi:BPMNShape>
                    <bpmndi:BPMNShape id="ExclusiveGateway_0k39v3u_di" bpmnElement="ExclusiveGateway_0k39v3u" isMarkerVisible="true">
                        <dc:Bounds x="525" y="275" width="50" height="50" />
                    </bpmndi:BPMNShape>
                    <bpmndi:BPMNEdge id="SequenceFlow_11h4o22_di" bpmnElement="SequenceFlow_11h4o22">
                        <di:waypoint x="550" y="230" />
                        <di:waypoint x="550" y="275" />
                    </bpmndi:BPMNEdge>
                    <bpmndi:BPMNEdge id="SequenceFlow_1iu7pfe_di" bpmnElement="SequenceFlow_1iu7pfe">
                        <di:waypoint x="575" y="300" />
                        <di:waypoint x="680" y="300" />
                        <di:waypoint x="680" y="380" />
                    </bpmndi:BPMNEdge>
                    <bpmndi:BPMNShape id="UserTask_18pwui1_di" bpmnElement="Task_10fqcwp">
                        <dc:Bounds x="630" y="380" width="100" height="80" />
                    </bpmndi:BPMNShape>
                    <bpmndi:BPMNEdge id="SequenceFlow_04uqww2_di" bpmnElement="SequenceFlow_04uqww2">
                        <di:waypoint x="525" y="300" />
                        <di:waypoint x="430" y="300" />
                        <di:waypoint x="430" y="380" />
                    </bpmndi:BPMNEdge>
                    <bpmndi:BPMNShape id="UserTask_1j0us24_di" bpmnElement="Task_15n23yh">
                        <dc:Bounds x="380" y="380" width="100" height="80" />
                    </bpmndi:BPMNShape>
                    <bpmndi:BPMNShape id="ExclusiveGateway_1sq33g6_di" bpmnElement="ExclusiveGateway_1sq33g6" isMarkerVisible="true">
                        <dc:Bounds x="525" y="515" width="50" height="50" />
                    </bpmndi:BPMNShape>
                    <bpmndi:BPMNEdge id="SequenceFlow_0c8wrs4_di" bpmnElement="SequenceFlow_0c8wrs4">
                        <di:waypoint x="430" y="460" />
                        <di:waypoint x="430" y="540" />
                        <di:waypoint x="525" y="540" />
                    </bpmndi:BPMNEdge>
                    <bpmndi:BPMNEdge id="SequenceFlow_1xod8nh_di" bpmnElement="SequenceFlow_1xod8nh">
                        <di:waypoint x="680" y="460" />
                        <di:waypoint x="680" y="540" />
                        <di:waypoint x="575" y="540" />
                    </bpmndi:BPMNEdge>
                    <bpmndi:BPMNShape id="EndEvent_0pnmjd3_di" bpmnElement="EndEvent_0pnmjd3">
                        <dc:Bounds x="532" y="602" width="36" height="36" />
                    </bpmndi:BPMNShape>
                    <bpmndi:BPMNEdge id="SequenceFlow_0h8za82_di" bpmnElement="SequenceFlow_0h8za82">
                        <di:waypoint x="550" y="565" />
                        <di:waypoint x="550" y="602" />
                    </bpmndi:BPMNEdge>
                    </bpmndi:BPMNPlane>
                </bpmndi:BPMNDiagram>
                </bpmn2:definitions>
            `
