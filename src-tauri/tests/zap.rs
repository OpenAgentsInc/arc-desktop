#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose, Engine as _};

    #[test]
    fn test_zap() {
        let zap_json = "eyJpZCI6IjUzNmJlZTllODNjODE4ZTNiODJjMTAxOTM1MTI4YWUyN2EwZDQyOTAwMzlhYWYyNTNlZmU1ZjA5MjMyYzE5NjIiLCJwdWJrZXkiOiI5NjMwZjQ2NGNjYTZhNTE0N2FhOGEzNWYwYmNkZDNjZTQ4NTMyNGU3MzJmZDM5ZTA5MjMzYjFkODQ4MjM4ZjMxIiwiY3JlYXRlZF9hdCI6MTY3NDIwNDUzNSwia2luZCI6OTczNSwidGFncyI6W1sicCIsIjMyZTE4Mjc2MzU0NTBlYmIzYzVhN2QxMmMxZjhlN2IyYjUxNDQzOWFjMTBhNjdlZWYzZDlmZDljNWM2OGUyNDUiXSxbImJvbHQxMSIsImxuYmMxMHUxcDN1NTR0bnNwNTcyOXF2eG5renRqamtkNTg1eW4wbDg2MzBzMm01eDZsNTZ3eXk0ZWMybnU4eHV6NjI5eHFwcDV2MnE3aHVjNGpwamgwM2Z4OHVqZXQ1Nms3OWd4cXg3bWUycGV2ejZqMms4dDhtNGxnNXZxaHA1eWc1MDU3OGNtdWoyNG1mdDNxcnNybWd3ZjMwa2U3YXY3ZDc3Z2FtZmxkazlrNHNmMzltcXhxeWp3NXFjcXBqcnpqcTJoeWVoNXEzNmx3eDZ6dHd5cmw2dm1tcnZ6NnJ1ZndqZnI4N3lremZuYXR1a200dWRzNHl6YWszc3FxOW1jcXFxcXFxcWxncXFxcTg2cXF5ZzlxeHBxeXNncWFkeWVjdmR6ZjI3MHBkMzZyc2FmbDA3azQ1ZmNqMnN5OGU1djJ0ZW5kNTB2OTU3NnV4cDNkdmp6amV1aHJlODl5cGdjbTkwZDZsbTAwNGszMHlqNGF2NW1jc3M1bnl4NHU5bmVyOWdwcHY2eXF3Il0sWyJkZXNjcmlwdGlvbiIsIntcImlkXCI6XCJiMDkyMTYzNGIxYmI4ZWUzNTg0YmJiZjJlOGQ3OTBhZDk4NTk5ZDhlMDhmODFjNzAwZGRiZTQ4MjAxNTY4Yjk3XCIsXCJwdWJrZXlcIjpcIjdmYTU2ZjVkNjk2MmFiMWUzY2Q0MjRlNzU4YzMwMDJiODY2NWY3YjBkOGRjZWU5ZmU5ZTI4OGQ3NzUxYWMxOTRcIixcImNyZWF0ZWRfYXRcIjoxNjc0MjA0NTMxLFwia2luZFwiOjk3MzQsXCJ0YWdzXCI6W1tcInBcIixcIjMyZTE4Mjc2MzU0NTBlYmIzYzVhN2QxMmMxZjhlN2IyYjUxNDQzOWFjMTBhNjdlZWYzZDlmZDljNWM2OGUyNDVcIl0sW1wicmVsYXlzXCIsXCJ3c3M6Ly9yZWxheS5zbm9ydC5zb2NpYWxcIixcIndzczovL3JlbGF5LmRhbXVzLmlvXCIsXCJ3c3M6Ly9ub3N0ci1wdWIud2VsbG9yZGVyLm5ldFwiLFwid3NzOi8vbm9zdHIudjBsLmlvXCIsXCJ3c3M6Ly9wcml2YXRlLW5vc3RyLnYwbC5pb1wiLFwid3NzOi8vbm9zdHIuemViZWRlZS5jbG91ZFwiLFwid3NzOi8vcmVsYXkubm9zdHIuaW5mby9cIl1dLFwiY29udGVudFwiOlwiXCIsXCJzaWdcIjpcImQwODQwNGU2MjVmOWM1NjMzYWZhZGQxMWMxMTBiYTg4ZmNkYjRiOWUwOTJiOTg0MGU3NDgyYThkNTM3YjFmYzExODY5MmNmZDEzMWRkODMzNTM2NDc2OWE2NzE3NTRhZDdhYTk3MzEzNjgzYTRhZDdlZmI3NjQ3NmMwNGU1ZjE3XCJ9Il0sWyJwcmVpbWFnZSIsIjNlMDJhM2FmOGM4YmNmMmEzNzUzYzg3ZjMxMTJjNjU2YTIwMTE0ZWUwZTk4ZDgyMTliYzU2ZjVlOGE3MjM1YjMiXV0sImNvbnRlbnQiOiIiLCJzaWciOiIzYWI0NGQwZTIyMjhiYmQ0ZDIzNDFjM2ZhNzQwOTZjZmY2ZjU1Y2ZkYTk5YTVkYWRjY2Y0NWM2NjQ2MzdlMjExNTFiMmY5ZGQwMDQwZjFhMjRlOWY4Njg2NzM4YjE2YmY4MTM0YmRiZTQxYTIxOGM5MTFmN2JiMzFlNTk1NzhkMSJ9Cg=="; // base64-encoded JSON string goes here

        let zap_bytes = general_purpose::STANDARD.decode(zap_json).unwrap();

        let zap = match Zap::from_zap_event(&zap_bytes) {
            Some(zap) => zap,
            None => {
                // If parsing fails, panic and print an error message
                panic!("Failed to parse Zap object");
            }
        };

        // Assert that the `zapper` field of `zap` matches the expected value
        assert_eq!(
            zap.zapper,
            "9630f464cca6a5147aa8a35f0bcdd3ce485324e732fd39e09233b1d848238f31"
        );

        // Assert that the `target` field of `zap` matches the expected value
        assert_eq!(
            zap.target,
            ZapTarget::Profile(
                "32e1827635450ebb3c5a7d12c1f8e7b2b514439ac10a67eef3d9fd9c5c68e245".into()
            )
        );
        // Assert that the `timestamp_created_at` field of `zap` matches the expected value
        assert_eq!(zap.timestamp_created_at, 1_674_204_533);

        // Assert that the `kind` field of `zap` matches the expected value
        assert_eq!(zap.kind, "9735");

        // Assert that the `tags` field of `zap` matches the expected value
        assert_eq!(
            zap.tags,
            vec![
                "p",
                "32e1827635450ebb3c5a7d12c1f8e7b2b514439ac10a67eef3d9fd9cd5c68e245",
                "[\"both11\",\"lnbc10u1p3u54tnsp5729qvxnzkztjjdk585ynl86s2m5x6l56wyy4ec2nu8xuz629xqpwp5v2q7huc4jpjh03fx8ujet56k79gxqx7me2pevz6j2k8t8m4lg5vqhp5yg50578cmuj24mft3qrsrmgwf30ke7av7d77gamfldk9ksf39mqxqyjw5qcqpjrjq2hyeh5q36lwxx6ztwyrl6vmmrvz6rufwjfr87ykzkfnaturm4uds4yza3sqo9mjcqqxxgqgzqpgcmt90d6lm004k30yj4av5mcs5nyx4u9ner9gppv6qqw"
            ]
        );
    }
}
