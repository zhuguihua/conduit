package cmd

import (
	"context"
	"errors"
	"fmt"

	"github.com/runconduit/conduit/controller/api/public"
	pb "github.com/runconduit/conduit/controller/gen/public"
	"github.com/runconduit/conduit/pkg/k8s"
	"github.com/runconduit/conduit/pkg/shell"
	"github.com/spf13/cobra"
)

var getCmd = &cobra.Command{
	Use:   "get [flags] RESOURCE",
	Short: "Display one or many mesh resources",
	Long: `Display one or many mesh resources.

Valid resource types include:
 * pods (aka pod, po)`,
	RunE: func(cmd *cobra.Command, args []string) error {
		if len(args) < 1 {
			return errors.New("please specify a resource type")
		}

		if len(args) > 1 {
			return errors.New("please specify only one resource type")
		}

		friendlyName := args[0]
		resourceType, err := k8s.CanonicalKubernetesNameFromFriendlyName(friendlyName)

		if err != nil || resourceType != k8s.KubernetesPods {
			return fmt.Errorf("invalid resource type %s, only %s are allowed as resource types", friendlyName, k8s.KubernetesPods)
		}

		kubeApi, err := k8s.NewK8sAPi(shell.NewUnixShell(), kubeconfigPath, apiAddr)
		if err != nil {
			return err
		}

		client, err := newApiClient(kubeApi)
		if err != nil {
			return err
		}

		podNames, err := getPods(client)
		if err != nil {
			return err
		}

		for _, podName := range podNames {
			fmt.Println(podName)
		}

		return nil
	},
}

func init() {
	RootCmd.AddCommand(getCmd)
	addControlPlaneNetworkingArgs(getCmd)
}

func getPods(apiClient public.ConduitApiClient) ([]string, error) {
	resp, err := apiClient.ListPods(context.Background(), &pb.Empty{})
	if err != nil {
		return nil, err
	}

	names := make([]string, 0)
	for _, pod := range resp.GetPods() {
		names = append(names, pod.Name)
	}

	return names, nil
}
