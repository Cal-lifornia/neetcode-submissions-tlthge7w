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
        while(idx < nums.size()) {
            while(nums[idx] == val && idx < nums.size()) {
                removeEntry(nums,idx,nums.size());
            }
            idx++;
        }
        return nums.size();
    }
};