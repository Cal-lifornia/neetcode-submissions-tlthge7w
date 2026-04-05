class Solution {
public:
    void removeEntry(vector<int>& nums, int idx, int length) {
        for(int index = idx +1; index<length; index++) {
            nums[index - 1] = nums[index];
        }
        nums.pop_back();
    }
    int removeElement(vector<int>& nums, int val) {
        int idx = 0;
        int length = nums.size();
        while(idx < length) {
            while(nums[idx] == val && idx < length) {
                removeEntry(nums,idx,length);
                length = nums.size();
            }
            idx++;
        }
        return length;
    }
};