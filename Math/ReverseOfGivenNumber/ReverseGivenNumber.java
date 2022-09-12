public class ReverseGivenNumber {
    public static void main(String[] args) {
        // logic
        // int n
        // rev = 0
        // lastDigit = n % 10
        // rev = (rev * 10) + lastDigit
        int n = 10899;
        int rev = 0;

        while (n > 0) {
            int lastDigit = n % 10;
            rev = (rev * 10) + lastDigit;
            n = n / 10;
        }

        System.out.println(rev);
    }
}
