//infix  to postfix conversion using stack

import java.io.*;
import java.util.*;

public class InfixToPostfix {
	public static void main (String[] args) throws IOException {
		BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		int t = Integer.parseInt(br.readLine().trim());	//read number of test case
		while(t-->0){
		    System.out.println(new solve().infixToPostfix(br.readLine().trim()));	//read infix string and print postfix string
		}
	}
}
class solve{
	public static String infixToPostfix(String exp) {
		String str = "";
		Stack<Character> stack = new Stack<>();
		//if character is letter or digit add character to new string
		// if character is opening braces push in stack
		// if character is closing braces pop from stack while stack peek not equal to opening braces
		// if stack is not empty and peek not equal to opening braces return invalid expression else pop from stack
		// if character is an operator call precidency method.
		for (int i = 0; i < exp.length(); i++) {
			char c = exp.charAt(i);
			if (Character.isLetterOrDigit(c)) {
				str+=c;
			} else if (c=='(') {
				stack.push(c);
			} else if (c==')') {
				while(!stack.isEmpty() && stack.peek()!='(') {
					str+=stack.pop();
				}
				if (!stack.isEmpty() && stack.peek()!='(') {
					return "Invalid Expression";
				} else {
					stack.pop();
				}
			} else {
				while(!stack.isEmpty() && precidence(c)<=precidence(stack.peek())) {
					if (stack.peek()=='(') {
						return "Invalid Expression";
					}
					str += stack.pop();
				}
				stack.push(c);
			}
		}
		while (!stack.isEmpty()) {
			if (stack.peek()=='(') {
				return "Invalid Expression";
			}
			str += stack.pop();
		}
		return str;
	}
	
	private static int precidence(Character ch) {
		// TODO Auto-generated method stub
		switch (ch) {
		case '+':
		case '-':
			return 1;
		case '*':
		case '/':
			return 2;
		case '^':
			return 3;
		}
		return -1;
	} 
}