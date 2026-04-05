class Solution {
public:
    void removeEntry(vector<int>& nums, int idx, int length) {
        for(int index = idx +1; index<length; index++) {
            nums[index - 1] = nums[index];
        }
        nums.pop_back();
    }
    int removeDuplicates(vector<int>& nums) {
        int i = 0;
        while (i < nums.size()) {
            int current_num = i + 1;
            while (nums[current_num] == nums[i] && current_num < nums.size()) {
                removeEntry(nums, current_num, nums.size());
            }
            i++;
        } 
        return nums.size();
    }
};

