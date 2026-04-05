/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
public:
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        if (!list1) return list2;
        if (!list2) return list1;
        
        ListNode* dummy = new ListNode();
        ListNode* sorted = dummy;
        ListNode* prev = list1;
        ListNode* current = list2;

        while (prev!= nullptr && current != nullptr) {
            if (current->val > prev->val) {
                sorted->next = prev;
                prev = prev->next;
            } else {
                sorted->next = current;
                current = current->next;
            }
            sorted = sorted->next;
        }
        if (prev != nullptr) {
            sorted->next = prev;
        }
        if (current != nullptr) {
            sorted->next = current;
        }
        return dummy->next;
    }
};