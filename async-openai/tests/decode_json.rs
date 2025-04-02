use async_openai::types::CreateChatCompletionResponse;

#[test]
fn test_decode_json() {
    let json_str = r#"
        {
	        "choices": [{
	        	"finish_reason": "stop",
	        	"index": 0,
	        	"message": {
	        		"content": "当然可以。下面是一的冒泡排序算法的示例：\n\n```c\n#include <stdio.h>\n\nvoid bubbleSort(int arr[], int n) {\n    int i, j, temp;\n    for (i = 0; i < n-1; i++) {     \n        // 最后i个要再次检查\n        for (j = 0; j < n-i-1; j++) {\n            if (arr[j] > arr[j+1]) {\n                // 交换两个元素的位置\n                temp = arr[j];\n       ] = arr[j+1];\n                arr[j+1] = temp;\n            }\n        }\n    }\n}\n\nvoid printArray(int arr[], int size) {\n    int i;\n    for (i = 0; i < size; i++) {\n        printf(\"%d \", arr[i]);\n    }\n    printf(\"\\n\");\n}\n\nint main() {\n    int arr[] = {64, 34, 25, 12, 22, 11, 90};\n    int n = sizeof(arr)/sizeof(arr[0]);\n    bubbleSort(arr, n);\n    printf(\"Sorted array: \\n\");\n    printArray(arr, n);\n    return 0;\n}\n```\n\n在这个程序中，`bubbleSort` 函数负责执行冒泡排序于打印数组内容。在 `main` 函数中，我们定义了一个整数数组 `arr`，然后调用 `bubbleSort` 对其进行排序，最后使用 `printArray` 打印排序后的数组。\n\n冒泡排序的工作原理是通过也就是说该数列已经排序完成。这个算法的名字由来是因为越小的元素会经由交换慢慢“浮”到数列的顶端。",
	        		"role": "assistant"
	        	}
	        }],
	        "created": 1743580607,
	        "id": "2025040215562890aca1360215562890aca1362fc643a2",
	        "usage": {
	        	"completion_tokens": 406,
	        	"prompt_tokens": 151,
	        	"total_tokens": 557
	        }
        }
    "#;
    
    let _: CreateChatCompletionResponse = serde_json::from_str(json_str).unwrap();
}