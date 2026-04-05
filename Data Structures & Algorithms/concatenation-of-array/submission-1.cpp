class Solution {
public:
    vector<int> getConcatenation(vector<int>& nums) {
        int current_size = nums.size();
        std::vector<int>ans(current_size * 2);
        for (int idx = 0; idx < current_size; idx++) {
            ans[idx] = nums[idx];
            ans[idx + current_size] = nums[idx];
        }
        return ans;
    }
};