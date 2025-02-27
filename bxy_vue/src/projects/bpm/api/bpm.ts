import { http } from "@/toolkit"
import type { HttpRequestData } from "@/types"

/**
 * 加载bpmn
 * @param params
 * @returns
 */
export const GetBpmn= (params: HttpRequestData) =>
{
  return http.get<any>('/api/v1/bpm/bpmn/find_by_id', params)
}

/**
 * 保存bpmn
 * @param params
 * @returns
 */
export const SaveBpmn = (params: HttpRequestData) => {
  return http.get<any>('/api/v1/bpm/bpmn/save', params)
}
